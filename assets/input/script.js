document.addEventListener('DOMContentLoaded', () => {
    const colorMap = {
        0: "red",
        1: "orange",
        2: "yellow",
        3: "green",
        4: "blue",
        5: "purple",
        6: "pink",
        7: "black",
    };

    console.log('Script loaded'); // Confirm script loading

    let scaleFactor = 1.2;  // Defines the zoom factor
    let currentScale = 1;   // Keeps track of the current zoom level
    let originX = 0;        // X-coordinate for zoom origin
    let originY = 0;        // Y-coordinate for zoom origin

    const canvas = document.getElementById("myCanvas");
    const context = canvas.getContext("2d");
    const circles = [];
    let paths = [];

    const image = document.getElementById("hallwayImage");

    document.getElementById("zoomIn").addEventListener("click", () => zoomCanvas(scaleFactor));
    document.getElementById("zoomOut").addEventListener("click", () => zoomCanvas(1 / scaleFactor));

    function zoomCanvas(factor) {
        currentScale *= factor;

        const rect = canvas.getBoundingClientRect();

        // Adjust origin to zoom towards the center of the canvas
        originX = (rect.width / 2 - (rect.width / 2 - originX * factor));
        originY = (rect.height / 2 - (rect.height / 2 - originY * factor));

        // Clear the canvas before applying new transformation
        context.setTransform(currentScale, 0, 0, currentScale, originX, originY);
        context.clearRect(-originX, -originY, canvas.width, canvas.height);

        // Redraw all elements
        redrawAll();
    }

    function redrawAll() {
        context.clearRect(0, 0, canvas.width, canvas.height);
        redrawPaths(context, paths);
        redrawCircles(context, circles);
        syncImageTransform();
    }

    function syncImageTransform() {
        // Ensure the image's transformation matches the canvas' current scale and position
        image.style.transform = `scale(${currentScale})`;
        image.style.transformOrigin = `0 0`;
        image.style.left = `${originX}px`;
        image.style.top = `${originY}px`;
    }

    document.getElementById('scheduleForm').addEventListener('submit', function (e) {
        e.preventDefault(); // Prevent the default form submission action
        console.log('Form submitted'); // Confirm form submission is intercepted

        const scheduleInput = document.getElementById('scheduleInput').value;
        const selectedDay = document.getElementById('daySelector').value;

        circles.length = 0; // Clear any previous circles
        paths = []; // Clear any previous paths

        fetch("/schedule-post", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                "Schedule Input": scheduleInput,
            }),
        })
            .then((data) => data.json())
            .then((json) => {
                if (json.status == 1) {
                    console.log("Couldn't get path sorry :I");
                    return;
                }
                const xShift = 3; // Shift in the x direction
                const yShift = 3; // Shift in the y direction

                context.clearRect(0, 0, canvas.width, canvas.height);

                let curday = json[selectedDay];
                curday.forEach((path, curnum) => {
                    const pathPoints = [];
                    for (let i = 1; i < path.length; i++) {
                        const startX = path[i - 1]["x"] + curnum * xShift;
                        const startY = path[i - 1]["y"] + curnum * yShift;
                        const endX = path[i]["x"] + curnum * xShift;
                        const endY = path[i]["y"] + curnum * yShift;

                        context.beginPath();
                        context.moveTo(startX, startY);
                        context.lineTo(endX, endY);
                        context.strokeStyle = colorMap[curnum];
                        context.lineWidth = 3;
                        context.stroke();
                        context.closePath();

                        // Store the path points for redrawing later
                        pathPoints.push({ startX, startY, endX, endY, color: colorMap[curnum] });

                        // Draw a small circle at the starting point where the line changes direction
                        context.beginPath();
                        context.arc(startX, startY, 4, 0, 2 * Math.PI, false);
                        context.fillStyle = colorMap[curnum];
                        context.fill();
                        context.closePath();

                        // Store circle data
                        circles.push({
                            x: startX,
                            y: startY,
                            radius: 4,
                            name: path[i - 1]["name"],
                            color: colorMap[curnum],
                        });
                    }

                    // Ensure the last point is handled correctly
                    if (path.length > 1) {
                        const finalX = path[path.length - 1]["x"] + curnum * xShift;
                        const finalY = path[path.length - 1]["y"] + curnum * yShift;

                        // Store only the final point without drawing an extra line
                        pathPoints.push({ startX: finalX, startY: finalY, endX: finalX, endY: finalY, color: colorMap[curnum] });

                        context.beginPath();
                        context.arc(finalX, finalY, 4, 0, 2 * Math.PI, false);
                        context.fillStyle = colorMap[curnum];
                        context.fill();
                        context.closePath();

                        circles.push({
                            x: finalX,
                            y: finalY,
                            radius: 4,
                            name: path[path.length - 1]["name"],
                            color: colorMap[curnum],
                        });
                    }

                    // Save the entire path for later redrawing
                    paths.push(pathPoints);
                });

                redrawAll();
            })
            .catch((err) => {
                console.log("Fetch error:", err);
            });
    });

    canvas.addEventListener('mousemove', function (e) {
        const { mouseX, mouseY } = getMousePosition(e, canvas);
        redrawAll(); // Clear and redraw the canvas

        circles.forEach(circle => {
            if (isMouseOverCircle(mouseX, mouseY, circle)) {
                context.font = "28px Arial";
                context.fillStyle = circle.color;
                context.fillText(circle.name, circle.x * currentScale + 10, circle.y * currentScale + 5); // Display the name when hovering
            }
        });
    });

    function getMousePosition(event, canvas) {
        const rect = canvas.getBoundingClientRect();
        const scaleX = canvas.width / rect.width / currentScale;
        const scaleY = canvas.height / rect.height / currentScale;
        return {
            mouseX: (event.clientX - rect.left) * scaleX - originX / currentScale,
            mouseY: (event.clientY - rect.top) * scaleY - originY / currentScale
        };
    }

    function isMouseOverCircle(mouseX, mouseY, circle) {
        const distance = Math.sqrt((mouseX - circle.x) ** 2 + (mouseY - circle.y) ** 2);
        return distance <= circle.radius;
    }

    function redrawPaths(context, paths) {
        paths.forEach(path => {
            path.forEach((segment, index) => {
                // Only draw if start and end points are different
                if (segment.startX !== segment.endX || segment.startY !== segment.endY) {
                    context.beginPath();
                    context.moveTo(segment.startX, segment.startY);
                    context.lineTo(segment.endX, segment.endY);
                    context.strokeStyle = segment.color;
                    context.lineWidth = 3;
                    context.stroke();
                    context.closePath();
                }

                // Avoid drawing an unnecessary line at the last node
                if (index === path.length - 1) {
                    context.beginPath();
                    context.arc(segment.endX, segment.endY, 4, 0, 2 * Math.PI, false);
                    context.fillStyle = segment.color;
                    context.fill();
                    context.closePath();
                }
            });
        });
    }

    function redrawCircles(context, circles) {
        circles.forEach(circle => {
            context.beginPath();
            context.arc(circle.x, circle.y, circle.radius, 0, 2 * Math.PI, false);
            context.fillStyle = circle.color;
            context.fill();
            context.closePath();
        });
    }
});
