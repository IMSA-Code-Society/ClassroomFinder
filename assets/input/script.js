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

    let scaleFactor = 1.2;
    let currentScale = 1;
    let originX = 0;
    let originY = 0;

    const canvas = document.getElementById("myCanvas");
    const context = canvas.getContext("2d");
    const circles = [];
    let paths = [];

    const image = document.getElementById("hallwayImage");

    document.getElementById("zoomIn").addEventListener("click", () => zoomCanvas(scaleFactor));
    document.getElementById("zoomOut").addEventListener("click", () => zoomCanvas(1 / scaleFactor));

    document.addEventListener('keydown', (e) => {
        if (e.key === '+') {
            zoomCanvas(scaleFactor);
        } else if (e.key === '-') {
            zoomCanvas(1 / scaleFactor);
        }
    });

    function zoomCanvas(factor) {
        currentScale *= factor;

        const rect = canvas.getBoundingClientRect();

        originX = (rect.width / 2 - (rect.width / 2 - originX * factor));
        originY = (rect.height / 2 - (rect.height / 2 - originY * factor));

        context.setTransform(1, 0, 0, 1, 0, 0);
        context.clearRect(0, 0, canvas.width, canvas.height);
        context.setTransform(currentScale, 0, 0, currentScale, originX, originY);
        redrawAll();
    }

    function redrawAll() {
        context.clearRect(0, 0, canvas.width, canvas.height);
        redrawPaths(context, paths);
        redrawCircles(context, circles);
        syncImageTransform();
    }

    function syncImageTransform() {
        image.style.transform = `scale(${currentScale})`;
        image.style.transformOrigin = `0 0`;
        image.style.left = `${originX}px`;
        image.style.top = `${originY}px`;
    }

    document.getElementById('scheduleForm').addEventListener('submit', function (e) {
        e.preventDefault();

        const scheduleInput = document.getElementById('scheduleInput').value;
        const selectedDay = document.getElementById('daySelector').value;

        circles.length = 0;
        paths = [];

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
                    return;
                }
                console.log(json);
                const xShift = 3;
                const yShift = 3;

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

                        pathPoints.push({ startX, startY, endX, endY, color: colorMap[curnum] });

                        context.beginPath();
                        context.arc(startX, startY, 4, 0, 2 * Math.PI, false);
                        context.fillStyle = colorMap[curnum];
                        context.fill();
                        context.closePath();

                        circles.push({
                            x: startX,
                            y: startY,
                            radius: 4,
                            name: path[i - 1]["name"],
                            color: colorMap[curnum],
                        });
                    }

                    if (path.length > 1) {
                        const finalX = path[path.length - 1]["x"] + curnum * xShift;
                        const finalY = path[path.length - 1]["y"] + curnum * yShift;

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

                    paths.push(pathPoints);
                });

                redrawAll();
            })
            .catch((err) => {
                console.log("Fetch error:", err);
            });
    });

    const tooltip = document.getElementById('tooltip');

    function getMousePosition(event, canvas) {
        const rect = canvas.getBoundingClientRect();
        const scaleX = canvas.width / rect.width;
        const scaleY = canvas.height / rect.height;

        const canvasX = (event.clientX - rect.left) * scaleX;
        const canvasY = (event.clientY - rect.top) * scaleY;

        return {
            mouseX: (canvasX - originX) / currentScale,
            mouseY: (canvasY - originY) / currentScale
        };
    }

    canvas.addEventListener('mousemove', function (e) {
        const { mouseX, mouseY } = getMousePosition(e, canvas);
        let tooltipVisible = false;

        circles.forEach(circle => {
            if (isMouseOverCircle(mouseX, mouseY, circle)) {
                tooltip.style.display = 'block';

                const rect = canvas.getBoundingClientRect();
                const scaleX = canvas.width / rect.width;
                const scaleY = canvas.height / rect.height;

                const tooltipX = e.clientX + window.scrollX;
                const tooltipY = e.clientY + window.scrollY;

                tooltip.style.left = `${tooltipX}px`;
                tooltip.style.top = `${tooltipY}px`;
                tooltip.innerHTML = `
                <div style="border: 1px solid ${circle.color}; background-color: ${circle.color};">
                <strong>label: ${circle.name}</strong>
                </div>`;
                tooltipVisible = true;
            }
        });

        if (!tooltipVisible) {
            tooltip.style.display = 'none';
        }
    });

    function isMouseOverCircle(mouseX, mouseY, circle) {
        const distance = Math.sqrt((mouseX - circle.x) ** 2 + (mouseY - circle.y) ** 2);
        return distance <= circle.radius;
    }

    function redrawPaths(context, paths) {
        paths.forEach(path => {
            path.forEach((segment, index) => {
                if (segment.startX !== segment.endX || segment.startY !== segment.endY) {
                    context.beginPath();
                    context.moveTo(segment.startX, segment.startY);
                    context.lineTo(segment.endX, segment.endY);
                    context.strokeStyle = segment.color;
                    context.lineWidth = 3;
                    context.stroke();
                    context.closePath();
                }
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
