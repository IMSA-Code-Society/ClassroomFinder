
document.addEventListener('DOMContentLoaded', () => {
    const manual = 0.86;
    let currentScale = 1;
    document.getElementsByClassName("path_button")[0].addEventListener("click", () => {
        arrowFetch()
    })
    const svg = document.getElementById("mySvg");
    const image = document.getElementById("hallwayImage");
    image.onload = () => {
        svg.setAttribute("viewBox", `0 0 ${image.width} ${image.height}`);
    };
    function isStairNode(node) {
        return /Stair.*F[12]$/.test(node.name);
    }
    function createSvgElement(tag, attrs) {
        const element = document.createElementNS("http://www.w3.org/2000/svg", tag);
        for (let key in attrs) {
            element.setAttribute(key, attrs[key]);
        }
        return element;
    }
    function drawCircle(cx, cy, color) {
        const circle = createSvgElement("circle", {
            cx,
            cy,
            r: 8 / currentScale,
            fill: color,
            stroke: "black",
            "stroke-width": 1
        });
        svg.appendChild(circle);
    }
    function arrowFetch() {
        const scaleStr = `scale(${currentScale})`;
        svg.style.transform = scaleStr;
        image.style.transform = scaleStr;
        const container = document.getElementById('hallwayImage');
        if (!container) return console.error("hallwayImage container not found!")
        const { width, height } = container.getBoundingClientRect();
        if (!width || !height) return console.error("Container has zero width or height:", width, height);
        const starting_location = document.getElementById("start").value;
        const destination = document.getElementById("end").value;
        const offset = 1320 / width;

        fetch("/get_directions", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                "start-room": starting_location,
                destination: destination,
            }),
        })
            .then((data) => data.json())
            .then((json) => {

                console.log("Received JSON:", json);

                if (json.status == 1) {
                    document.getElementById("message").style.color = "red";
                    document.getElementById("message").innerHTML = "Error. Path could not be found.";
                    return;
                }

                document.getElementById("message").style.color = "green";
                document.getElementById("message").innerHTML = "Path found!";

                while (svg.firstChild) {
                    svg.removeChild(svg.firstChild);
                }

                const path = json.path;
                console.log("Drawing path:", path);
                let already_stair = false;
                for (let i = 1; i < path.length; i++) {
                    const x1 = (path[i - 1].x) * manual / currentScale / offset;
                    const y1 = (path[i - 1].y) * manual / currentScale / offset;
                    const x2 = (path[i].x) * manual / currentScale / offset;
                    const y2 = (path[i].y) * manual / currentScale / offset;
                    if (isStairNode(path[i - 1]) && !already_stair) {
                        already_stair = true;
                        drawCircle(x1, y1, "orange");
                        drawCircle(x2, y2, "orange");

                        function makeText(x, y, text) {
                            const SVG_NS = "http://www.w3.org/2000/svg";

                            const rect = document.createElementNS(SVG_NS, "rect");
                            rect.setAttribute("x", x1 + 5);
                            rect.setAttribute("y", y1 - 10);
                            rect.setAttribute("width", 120);
                            rect.setAttribute("height", 30);
                            rect.setAttribute("fill", "orange");
                            rect.setAttribute("fill-opacity", "0.5");

                            svg.appendChild(rect);

                            const label = document.createElementNS(SVG_NS, "text");
                            label.setAttribute("font-weight", "bold");
                            label.setAttribute("x", x1 + 10);
                            label.setAttribute("y", y1 + 5);
                            label.setAttribute("fill", "black");
                            label.setAttribute("font-size", "12");
                            label.textContent = text;
                            svg.appendChild(label);
                        }
                        if ((path[i].name).includes("F1")) {
                            makeText(x1, y1, "Take stairs down");
                        } else {
                            makeText(x1, y1, "Take stairs up");
                        }

                    }
                    else {

                        const line = document.createElementNS("http://www.w3.org/2000/svg", "line");
                        line.setAttribute("x1", x1);
                        line.setAttribute("y1", y1);
                        line.setAttribute("x2", x2);
                        line.setAttribute("y2", y2);
                        line.setAttribute("stroke", "green");
                        line.setAttribute("stroke-width", "5");
                        line.setAttribute("stroke-linecap", "round");
                        svg.appendChild(line);


                    }
                }


                adjustSvgSize();
            })
            .catch((err) => {
                console.log("Fetch error:", err);
            });
    };
    function adjustSvgSize() {
        const imageRect = image.getBoundingClientRect();
        svg.style.width = `${imageRect.width}px`;
        svg.style.height = `${imageRect.height}px`;
        svg.setAttribute("viewBox", `0 0 ${imageRect.width} ${imageRect.height}`);
    }
    window.addEventListener('resize', () => {
        arrowFetch();

    });

})
