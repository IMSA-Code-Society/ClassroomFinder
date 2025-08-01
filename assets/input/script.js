

document.addEventListener('DOMContentLoaded', () => {

    document.getElementById('bottom-part').hidden = false;
    const colorMap = {
        0: "red",
        1: "orange",
        2: "yellow",
        3: "green",
        4: "blue",
        5: "purple",
        6: "pink",
        7: "grey",
        8: "brown",
        9: "black",
    };

    const scaleFactor = 1.2;
    let currentScale = 1;
    const manualScaleFactor = 0.86;
    const svg = document.getElementById("mySvg");

    const image = document.getElementById('hallwayImage');


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

    function toggleMinimize() {

        if (document.getElementById('bottom-part').hidden) {
            document.getElementById('bottom-part').hidden = false;
            document.getElementById('map').style.height = '75%';
        } else {
            document.getElementById('bottom-part').hidden = true;
            document.getElementById('map').style.height = '100%';
        }
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
        const container = document.getElementById('hallwayImage');
        if (!container) {
            console.error("hallwayImage container not found!");
            return;
        }

        const { width, height } = container.getBoundingClientRect();
        if (!width || !height) {
            console.error("Container has zero width or height:", width, height);
            return;
        }
        const x1 = arrow.x1Pct * width;
        const y1 = arrow.y1Pct * height;
        const x2 = arrow.x2Pct * width;
        const y2 = arrow.y2Pct * height;
        const distance = pointToLineDistance(mouseX, mouseY, x1, y1, x2, y2);

        const close = distance <= 5;


        return close;
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



    function getFullPathDescription(pathDetails, start) {
        //if start is true then the given path stuff is a start, if false, it is a end
        //0 should be start 1 end
        let val = 0;
        if (start === false) {
            val = 1
        };
        console.log(val);
        console.log("Trying to get full, here's the deets: ", pathDetails)
        const fullPath = pathDetails.path.map((point, index) => `${point.name}`).join(' -> ');
        if (pathDetails.info !== null) {
            const { days, end, long_name, mods, room, semester, short_name, start, teacher } = pathDetails.info[val];
            const daysFormatted = days.join(", ");
            const modsFormatted = mods.join(", ");
            const formattedString = `
                Course: ${long_name} (${short_name})<br>
                Instructor: ${teacher}<br>
                Room: ${room}<br>
                Semester: ${semester}<br>
                Days: ${daysFormatted}<br>
                Mods: ${modsFormatted}<br>
                Start Date: ${start}<br>
                End Date: ${end}<br>
            `.trim();
            return `Route: ${fullPath}.<br><br> ${formattedString}`;
        } else {
            return `Route: ${fullPath}.`;
        }


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
                if (arrow.type === 'start') {
                    tooltipContent += getFullPathDescription(arrow.pathDetails, true);
                }
                if (arrow.type === 'end') {
                    tooltipContent += getFullPathDescription(arrow.pathDetails, false);
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
    window.addEventListener('resize', () => {
        arrowFetch();

    });

    document.getElementById("zoomIn").addEventListener("click", () => zoomCanvas(scaleFactor));
    document.getElementById("viewTog").addEventListener("click", () => toggleMinimize());
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

        arrowFetch();



    });
    function arrowFetch() {
        const scaleStr = `scale(${currentScale})`;
        svg.style.transform = scaleStr;
        image.style.transform = scaleStr;
        const scheduleInput = document.getElementById('scheduleInput').value;
        const selectedDay = document.getElementById('daySelector').value;
        const semester_type = document.getElementById('semSelector').value;
        const enter = document.getElementById('enterSelector').value;
        const exit = document.getElementById('exitSelector').value;
        const checkbox = document.getElementById('midday');
        const isChecked = checkbox.checked;
        fetch("/schedule-post", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ "Schedule Input": scheduleInput, "Enter": enter, "Exit": exit, "LexMidday": isChecked }),
        })
            .then((data) => data.json())
            .then((json) => {
                console.log("Here is the json: ", json);
                if (json.status == 1) {
                    document.getElementById('error_message').innerHTML = `There was an error: ${json.error_message}`;
                    return;
                }

                const final_json = json[semester_type];
                const xShift = 5, yShift = 5;
                svg.innerHTML = '';
                arrows.length = 0;

                const curday = final_json[selectedDay];
                const segmentMap = new Map();
                const offsetSpacing = 3; // pixels

                function getSegmentKey(n1, n2) {
                    const id1 = `${n1.x},${n1.y}`;
                    const id2 = `${n2.x},${n2.y}`;
                    return [id1, id2].sort().join("|");
                }

                curday.forEach((path, curnum) => {
                    console.log("Current path: ", path);
                    const pathDetails = {
                        path: path["nodes"],
                        startName: path["nodes"][0].name,
                        endName: path["nodes"][path["nodes"].length - 1].name,
                        info: path["info"],
                    };

                    for (let i = 1; i < path["nodes"].length; i++) {
                        const container = document.getElementById('hallwayImage');
                        const { width, height } = container.getBoundingClientRect();

                        const node1 = path["nodes"][i - 1];
                        const node2 = path["nodes"][i];

                        const segKey = getSegmentKey(node1, node2);
                        const existingCount = segmentMap.get(segKey) || 0;
                        segmentMap.set(segKey, existingCount + 1);


                        const dx = node2.x - node1.x;
                        const dy = node2.y - node1.y;

                        const len = Math.hypot(dx, dy) || 1;
                        const dirX = dx / len;
                        const dirY = dy / len;

                        const orthoX = -dirY;
                        const orthoY = dirX;

                        const offset = offsetSpacing * existingCount;

                        const xOffset = orthoX * offset;
                        const yOffset = orthoY * offset;

                        const x1 = (node1.x + xOffset) * manualScaleFactor / currentScale;
                        const y1 = (node1.y + yOffset) * manualScaleFactor / currentScale;
                        const x2 = (node2.x + xOffset) * manualScaleFactor / currentScale;
                        const y2 = (node2.y + yOffset) * manualScaleFactor / currentScale;
                        arrows.push({
                            x1Pct: x1 / width,
                            y1Pct: y1 / height,
                            x2Pct: x2 / width,
                            y2Pct: y2 / height,
                            color: colorMap[curnum],
                            name: node2.name,
                            pathDetails,
                            num: i - 1,
                            type: i === 1 ? 'start' : i === path["nodes"].length - 1 ? 'end' : 'mid'
                        });
                    }
                });


                adjustSvgSize();
                document.getElementById('error_message').innerHTML = "";
                redrawArrows();

            })

    }




    const arrows = [];
    const tooltip = document.getElementById('tooltip');

    svg.addEventListener('mousemove', handleMouseMove);
    svg.addEventListener('mouseleave', () => tooltip.style.display = 'none');



    function redrawArrows() {
        const container = document.getElementById('hallwayImage');
        if (!container) return console.error("hallwayImage container not found!");

        const { width, height } = container.getBoundingClientRect();
        if (!width || !height) return console.error("Container has zero width or height:", width, height);

        const offset = 1320 / width;
        svg.innerHTML = '';

        let currentColor = null;
        let path = null;
        let d = '';

        const radius = 12 / currentScale;
        const shortenEnd = 6 / currentScale;
        for (let i = 0; i < arrows.length; i++) {
            const cur = arrows[i];
            const x = cur.x1Pct * width / offset;
            const y = cur.y1Pct * height / offset;

            const nextX = cur.x2Pct * width / offset;
            const nextY = cur.y2Pct * height / offset;


            if (cur.color !== currentColor) {
                if (path) {
                    path.setAttribute("d", d);
                    svg.appendChild(path);
                }

                path = document.createElementNS("http://www.w3.org/2000/svg", "path");
                path.setAttribute("fill", "none");
                path.setAttribute("stroke", cur.color);
                path.setAttribute("stroke-width", 4 / currentScale);
                path.setAttribute("stroke-linejoin", "round");
                path.setAttribute("stroke-linecap", "round");

                currentColor = cur.color;
                d = `M ${x} ${y}`;
                continue;
            }


            if (i > 0 && arrows[i - 1].color === cur.color) {
                const prev = arrows[i - 1];
                const px = prev.x1Pct * width / offset;
                const py = prev.y1Pct * height / offset;

                const dx1 = x - px;
                const dy1 = y - py;
                const len1 = Math.hypot(dx1, dy1);
                const ux1 = dx1 / len1;
                const uy1 = dy1 / len1;

                const dx2 = nextX - x;
                const dy2 = nextY - y;
                const len2 = Math.hypot(dx2, dy2);
                const ux2 = dx2 / len2;
                const uy2 = dy2 / len2;


                const corner1X = x - ux1 * radius;
                const corner1Y = y - uy1 * radius;

                const corner2X = x + ux2 * radius;
                const corner2Y = y + uy2 * radius;


                d += ` L ${corner1X} ${corner1Y}`;

                d += ` Q ${x} ${y} ${corner2X} ${corner2Y}`;
            }


            const dx = nextX - x;
            const dy = nextY - y;
            const len = Math.hypot(dx, dy);

            if (len > 0) {
                const ux = dx / len;
                const uy = dy / len;

                const trimmedX = nextX - ux * shortenEnd;
                const trimmedY = nextY - uy * shortenEnd;

                d += `L ${trimmedX} ${trimmedY}`;
            }


            drawArrowhead(x, y, nextX, nextY, cur.color, 1.3 / currentScale);
        }

        if (path) {
            path.setAttribute("d", d);
            svg.appendChild(path);
        }
    }


    function drawArrowhead(x1, y1, x2, y2, color, scale) {
        const angle = Math.atan2(y2 - y1, x2 - x1);
        const arrowLength = 12 * scale;

        const tipX = x2;
        const tipY = y2;

        const baseX1 = tipX - arrowLength * Math.cos(angle - Math.PI / 8);
        const baseY1 = tipY - arrowLength * Math.sin(angle - Math.PI / 8);

        const baseX2 = tipX - arrowLength * Math.cos(angle + Math.PI / 8);
        const baseY2 = tipY - arrowLength * Math.sin(angle + Math.PI / 8);

        const arrowHead = createSvgElement("polygon", {
            points: `${tipX},${tipY} ${baseX1},${baseY1} ${baseX2},${baseY2}`,
            fill: color,
            stroke: "black",
            "stroke-width": 1
        });

        svg.appendChild(arrowHead);
    }




});