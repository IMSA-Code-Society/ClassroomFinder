
document.addEventListener('DOMContentLoaded', () => {

    let has_submitted = false;
    const manual = 0.86;
    let currentScale = 1;
    document.getElementsByClassName("path_button")[0].addEventListener("click", () => {
        has_submitted = true;
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
        const room = document.getElementById("room").value;

        const offset = 1320 / width;

        fetch("/find_room_loc", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                "room": room,
            }),
        })
            .then((data) => data.json())
            .then((json) => {

                console.log("Received JSON:", json);

                if (json.status == 1) {
                    document.getElementById("message").style.color = "red";
                    document.getElementById("message").innerHTML = "Error. Room could not be found.";
                    return;
                }

                document.getElementById("message").style.color = "green";
                document.getElementById("message").innerHTML = "Room found!";

                while (svg.firstChild) {
                    svg.removeChild(svg.firstChild);
                }

                const room = json.room;
                console.log("Got room", room);


                const x1 = (room.x) * manual / currentScale / offset;
                const y1 = (room.y) * manual / currentScale / offset;


                drawCircle(x1, y1, "orange", 30);
                drawCircle(x1, y1, "red", 40);


                const map = document.getElementById("map");
                
                map.scrollTop = y1 - 200;





                adjustSvgSize();
                function drawCircle(cx, cy, color, r) {
                    const circle = createSvgElement("circle", {
                        cx,
                        cy,
                        r: r / currentScale,
                        fill: "none",
                        stroke: color,
                        "stroke-width": 2
                    });
                    svg.appendChild(circle);
                }

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
        if (has_submitted) {
            arrowFetch();
        }
    });

})
