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

    const scaleFactor = 1.2;
    let currentScale = 1;
    const manualScaleFactor = 1.25;

    const svg = document.getElementById("mySvg");
    const image = document.getElementById("hallwayImage");

    function adjustSvgSize() {
        const imageRect = image.getBoundingClientRect();
        svg.style.width = `${imageRect.width}px`;
        svg.style.height = `${imageRect.height}px`;
        svg.setAttribute("viewBox", `0 0 ${imageRect.width} ${imageRect.height}`);
    }

    function zoomCanvas(factor) {
        currentScale *= factor;

        const scaleStr = `scale(${currentScale})`;
        svg.style.transform = scaleStr;
        image.style.transform = scaleStr;
    }

    function drawArrow(x1, y1, x2, y2, color, pathDetails, isStart, isEnd) {
        const arrowLength = 12 * manualScaleFactor * currentScale;
        const arrowWidth = 8 * manualScaleFactor * currentScale;
        const lineWidth = 5 * manualScaleFactor * currentScale;

        const angle = Math.atan2(y2 - y1, x2 - x1);

        const arrowX1 = x2 - arrowLength * Math.cos(angle - Math.PI / 9);
        const arrowY1 = y2 - arrowLength * Math.sin(angle - Math.PI / 9);

        const arrowX2 = x2 - arrowLength * Math.cos(angle + Math.PI / 9);
        const arrowY2 = y2 - arrowLength * Math.sin(angle + Math.PI / 9);

        const line = createSvgElement("line", {
            x1, y1, x2, y2,
            stroke: color,
            "stroke-width": lineWidth
        });

        const arrowHead = createSvgElement("polygon", {
            points: `${x2},${y2} ${arrowX1},${arrowY1} ${arrowX2},${arrowY2}`,
            fill: color
        });

        svg.appendChild(line);
        svg.appendChild(arrowHead);

        // Save the line start and end points
        arrows.push({
            x1, // Start X-coordinate
            y1, // Start Y-coordinate
            x2, // End X-coordinate
            y2, // End Y-coordinate
            radius: 5, // Adjust radius as needed
            name: pathDetails.path[isStart ? 0 : pathDetails.path.length - 1].name,
            color,
            pathDetails,
            type: isStart ? 'start' : isEnd ? 'end' : 'mid',
        });
    }




    function createSvgElement(tag, attrs) {
        const element = document.createElementNS("http://www.w3.org/2000/svg", tag);
        for (let key in attrs) {
            element.setAttribute(key, attrs[key]);
        }
        return element;
    }

    function getMousePosition(event, svg) {
        const rect = svg.getBoundingClientRect();
        const scaleX = svg.width.baseVal.value / rect.width;
        const scaleY = svg.height.baseVal.value / rect.height;

        const svgX = (event.clientX - rect.left) * scaleX;
        const svgY = (event.clientY - rect.top) * scaleY;

        return {
            mouseX: svgX / currentScale,
            mouseY: svgY / currentScale
        };
    }

    function isMouseOverArrow(mouseX, mouseY, arrow) {
        const distance = pointToLineDistance(mouseX, mouseY, arrow.x1, arrow.y1, arrow.x2, arrow.y2);
        return distance <= arrow.radius + 5; // Adjust tolerance as needed
    }

    function pointToLineDistance(px, py, x1, y1, x2, y2) {
        const A = px - x1;
        const B = py - y1;
        const C = x2 - x1;
        const D = y2 - y1;

        const dot = A * C + B * D;
        const len_sq = C * C + D * D;
        const param = len_sq !== 0 ? dot / len_sq : -1;

        let xx, yy;

        if (param < 0) {
            xx = x1;
            yy = y1;
        } else if (param > 0) {
            xx = x2;
            yy = y2;
        } else {
            xx = x1 + param * C;
            yy = y1 + param * D;
        }

        const dx = px - xx;
        const dy = py - yy;
        return Math.sqrt(dx * dx + dy * dy);
    }



    function getFullPathDescription(pathDetails) {
        const fullPath = pathDetails.path.map((point, index) => `${point.name}`).join(' -> ');
        return `Route: ${fullPath}`;
    }

    function handleMouseMove(e) {
        const { mouseX, mouseY } = getMousePosition(e, svg);
        let tooltipVisible = false;

        arrows.forEach(arrow => {
            if (isMouseOverArrow(mouseX, mouseY, arrow)) {
                tooltip.style.display = 'block';
                tooltip.style.left = `${e.clientX + window.scrollX + 5}px`;
                tooltip.style.top = `${e.clientY + window.scrollY + 5}px`;

                let tooltipContent = `<strong>${arrow.name}</strong><br/>`;

                if (arrow.type === 'start') {
                    tooltipContent += `<em>Start of Route</em><br/>`;
                } else if (arrow.type === 'end') {
                    tooltipContent += `<em>End of Route</em><br/>`;
                }

                if (arrow.type === 'start' || arrow.type === 'end') {
                    tooltipContent += getFullPathDescription(arrow.pathDetails);
                }

                tooltip.innerHTML = `<div style="border: 1px solid ${arrow.color}; background-color: ${arrow.color};">${tooltipContent}</div>`;

                tooltipVisible = true;
            }
        });

        if (!tooltipVisible) {
            tooltip.style.display = 'none';
        }
    }

    adjustSvgSize();
    window.addEventListener('resize', adjustSvgSize);

    document.getElementById("zoomIn").addEventListener("click", () => zoomCanvas(scaleFactor));
    document.getElementById("zoomOut").addEventListener("click", () => zoomCanvas(1 / scaleFactor));

    document.addEventListener('keydown', (e) => {
        if (e.key === '+') {
            zoomCanvas(scaleFactor);
        } else if (e.key === '-') {
            zoomCanvas(1 / scaleFactor);
        }
    });

    document.getElementById('scheduleForm').addEventListener('submit', function (e) {
        e.preventDefault();

        const scheduleInput = document.getElementById('scheduleInput').value;
        const selectedDay = document.getElementById('daySelector').value;
        const semester_type = document.getElementById('semSelector').value;

        fetch("/schedule-post", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ "Schedule Input": scheduleInput }),
        })
            .then((data) => data.json())
            .then((json) => {
                if (json.status == 1) {
                    document.getElementById('error_message').innerHTML = `There was an error: ${json.error_message}`;
                    return;
                }

                const final_json = json[semester_type];
                const xShift = 10, yShift = 10;
                svg.innerHTML = '';
                arrows.length = 0;

                const curday = final_json[selectedDay];
                curday.forEach((path, curnum) => {
                    const pathDetails = {
                        path,
                        startName: path[0].name,
                        endName: path[path.length - 1].name
                    };

                    for (let i = 1; i < path.length; i++) {
                        const startX = path[i - 1]["x"] * manualScaleFactor * currentScale + curnum * xShift;
                        const startY = path[i - 1]["y"] * manualScaleFactor * currentScale + curnum * yShift;
                        const endX = path[i]["x"] * manualScaleFactor * currentScale + curnum * xShift;
                        const endY = path[i]["y"] * manualScaleFactor * currentScale + curnum * yShift;

                        drawArrow(startX, startY, endX, endY, colorMap[curnum], pathDetails, i === 1, i === path.length - 1);
                    }
                });

                adjustSvgSize();
                document.getElementById('error_message').innerHTML = "";
            })
            .catch((err) => {
                console.log("Fetch error:", err);
            });
    });

    const arrows = [];
    const tooltip = document.getElementById('tooltip');

    svg.addEventListener('mousemove', handleMouseMove);
    svg.addEventListener('mouseleave', () => tooltip.style.display = 'none');
});
