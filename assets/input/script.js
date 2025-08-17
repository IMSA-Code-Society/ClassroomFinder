
const ex1 = `Y25-26 Semester 1
Exp 	Trm 	Crs-Sec 	Course Name 	Teacher 	Room 	Enroll 	Leave
1-2(A,C) 	S1 	SCI603-1 	Biology: Molecular & Cellular 	O'Leary-Driscoll, Sarah 	B108 	08/04/2025 	01/05/2026
3(A-D) 	S1 	MAT331-1 	BC Calculus III 	Trimm, Anderson 	A135 	08/04/2025 	01/05/2026
4(A-D) 	S1 	CS255-2 	Elements of Computing Systems 1 	Campbell, Dan K. 	A152 	08/04/2025 	01/05/2026
5(A-B,D) 	S1 	ENG201a-2 	Literary Explorations III: American 	Kotlarczyk, Adam C 	A119 	08/04/2025 	01/05/2026
6(A-D) 	S1 	SCI445-2 	Modern Physics 	Hawker, Eric 	B115 	08/04/2025 	01/05/2026
SE(A-D) 	S1 	SE001-101 	Support & Engagement 	Staff, New 	N/A 	08/18/2025 	01/05/2026
7(A-B, D) 	S1 	HSS201i-2 	Revolutions 	Buck, Patrick D. 	A114 	08/04/2025 	01/05/2026
8(A-D) 	S1 	WLG150-101 	French V 	Kwiatkowski, Anne 	A121 	08/04/2025 	01/05/2026
RC(A-Sp) 	Y25-26 	SLD130-7 	Rolling Check 	Staff, Residential Life 		08/18/2025 	06/16/2026
CC(A-Sp) 	Y25-26 	SLD120-7 	Curfew Check 	Staff, Residential Life 		08/18/2025 	06/16/2026
SIR(I) 	S1 	SIR099-999 	SIR 	Staff, SIR 	B131 	08/04/2025 	01/05/2026
EVE(A-Sp) 	Y25-26 	SLD100-28 	Residential Life 	Brown, Tyson 		08/18/2025 	06/16/2026

Y25-26 Semester 2
Exp 	Trm 	Crs-Sec 	Course Name 	Teacher 	Room 	Enroll 	Leave
1(A-B,D) 	S2 	HSS202-1 	The World in the Twentieth Century 	Eysturlid, Lee 	A147 	01/05/2026 	06/16/2026
2(A,C-D) 	S2 	WEL312-1 	Dance 	Myers, Mary Jane 	F100A 	01/05/2026 	06/16/2026
3(A-D) 	S2 	CS260-1 	Elements of Computing Systems 2 	Campbell, Dan K. 	A133 	01/05/2026 	06/16/2026
4(A-D) 	S2 	SCI425-1 	Planetary Science 	Hawker, Eric 	B115 	01/05/2026 	06/16/2026
5(A-B,D) 	S2 	ENG341-4 	Gender Studies 	Ott, Ashley 	A117 	01/05/2026 	06/16/2026
SE(A-D) 	S2 	SE001-201 	Support & Engagement 	Staff, New 	N/A 	01/05/2026 	06/16/2026
7(A-D) 	S2 	MAT442-3 	Multi-Variable Calculus 	Fogel, Micah 	A155 	01/05/2026 	06/16/2026
8(A-D) 	S2 	WLG150-201 	French V 	Kwiatkowski, Anne 	A121 	01/05/2026 	06/16/2026
RC(A-Sp) 	Y25-26 	SLD130-7 	Rolling Check 	Staff, Residential Life 		08/18/2025 	06/16/2026
CC(A-Sp) 	Y25-26 	SLD120-7 	Curfew Check 	Staff, Residential Life 		08/18/2025 	06/16/2026
EVE(A-Sp) 	Y25-26 	SLD100-28 	Residential Life 	Brown, Tyson 		08/18/2025 	06/16/2026

`
const ex2 = `Y25-26 Semester 1
Exp     Trm     Crs-Sec     Course Name     Teacher     Room     Enroll     Leave
1-2(A)     S1     FAR416-1     Digital Photography     Symoniak, Joyce     E119     08/04/2025     01/05/2026
3(A-B, D)     S1     HSS325-3     Modern Economics     Kearney, Patrick     A115     08/04/2025     01/05/2026
4(A-D)     S1     CS255-2     Elements of Computing Systems 1     Campbell, Dan K.     A152     08/04/2025     01/05/2026
5-6(B,D)     S1     SCI235-1     Biochemistry     Ahrendt, Angela J     A207     08/04/2025     01/05/2026
5(C) 6(A,C)     S1     ENG355-2     Adaptation: Literature and Film     Rettberg, Eric     A119     08/04/2025     01/05/2026
SE(A-D)     S1     SE001-101     Support & Engagement     Staff, New     N/A     08/18/2025     01/05/2026
8(A-D)     S1     MAT801-1     Advanced Topics in Mathematics     Fogel, Micah     A151     08/04/2025     01/05/2026
RC(A-Sp)     Y25-26     SLD130-5     Rolling Check     Staff, Residential Life         08/18/2025     06/16/2026
CC(A-Sp)     Y25-26     SLD120-5     Curfew Check     Staff, Residential Life         08/18/2025     06/16/2026
EVE(A-Sp)     Y25-26     SLD100-20     Residential Life     Schlesser, Caleb         08/18/2025     06/16/2026

Y25-26 Semester 2
Exp     Trm     Crs-Sec     Course Name     Teacher     Room     Enroll     Leave
2(A,C-D)     S2     WEL312-1     Dance     Myers, Mary Jane     F100A     01/05/2026     06/16/2026
3(A-B,D)     S2     ENG365-5     Speculative Fiction Studies     Townsend, Tracy A     A113     01/05/2026     06/16/2026
4(A-D)     S2     CS260-2     Elements of Computing Systems 2     Campbell, Dan K.     A133     01/05/2026     06/16/2026
5-6(A,C)     S2     SCI626-1     Environmental Microbiology     Klimek, Desirae     B156     01/05/2026     06/16/2026
5(B,D) 6(B)     S2     HSS352-2     History of Technology and Culture     Smith, Eric R     A149     01/05/2026     06/16/2026
SE(A-D)     S2     SE001-201     Support & Engagement     Staff, New     N/A     01/05/2026     06/16/2026
8(A-D)     S2     MAT445-1     Theory of Analysis     Fogel, Micah     A155     01/05/2026     06/16/2026
RC(A-Sp)     Y25-26     SLD130-5     Rolling Check     Staff, Residential Life         08/18/2025     06/16/2026
CC(A-Sp)     Y25-26     SLD120-5     Curfew Check     Staff, Residential Life         08/18/2025     06/16/2026
EVE(A-Sp)     Y25-26     SLD100-20     Residential Life     Schlesser, Caleb         08/18/2025     06/16/2026`
const ex3 = `Y25-26 Semester 1
Exp     Trm     Crs-Sec     Course Name     Teacher     Room     Enroll     Leave
1(A-B,D)     S1     HSS201b-1     Conflict in World History     Eysturlid, Lee     A147     08/04/2025     01/05/2026
2(A,C-D)     S1     ENG201b-3     Literary Explorations III: British     Ott, Ashley     A116     08/04/2025     01/05/2026
3(A-D)     S1     CS255-1     Elements of Computing Systems 1     Campbell, Dan K.     A152     08/04/2025     01/05/2026
6(A-D)     S1     SCI445-2     Modern Physics     Hawker, Eric     B115     08/18/2025     01/05/2026
SE(A-D)     S1     SE001-101     Support & Engagement     Staff, New     N/A     08/18/2025     01/05/2026
7(A-D)     S1     MAT473-3     Linear Algebra     Brummet, Evan     A135     08/18/2025     01/05/2026
8(A-D)     S1     WLG250-101     Spanish V     Kaluza, Marta J     A131     08/04/2025     01/05/2026
RC(A-Sp)     Y25-26     SLD130-4     Rolling Check     Staff, Residential Life         08/18/2025     06/16/2026
CC(A-Sp)     Y25-26     SLD120-4     Curfew Check     Staff, Residential Life         08/18/2025     06/16/2026
SIR(I)     S1     SIR099-999     SIR     Staff, SIR     B131     08/04/2025     01/05/2026
EVE(A-Sp)     Y25-26     SLD100-14     Residential Life     Doxey, Cameron         08/18/2025     06/16/2026

Y25-26 Semester 2
Exp     Trm     Crs-Sec     Course Name     Teacher     Room     Enroll     Leave
1-2(B,D)     S2     SCI505-2     Computational Science     Dong, Peter J     IN2     01/05/2026     06/16/2026
3-4(A,C)     S2     SCI316-2     Electronics     Carlson, Mark     B133     01/05/2026     06/16/2026
3-4(B,D)     S2     SCI604-7     Biology: Molecular & Cellular     Klimek, Desirae     B108     01/05/2026     06/16/2026
5(A-B,D)     S2     HSS202-6     The World in the Twentieth Century     Eysturlid, Lee     A147     01/05/2026     06/16/2026
6(A,C-D)     S2     ENG365-3     Speculative Fiction Studies     Townsend, Tracy A     A117     01/05/2026     06/16/2026
SE(A-D)     S2     SE001-201     Support & Engagement     Staff, New     N/A     01/05/2026     06/16/2026
7(A-D)     S2     MAT442-3     Multi-Variable Calculus     Fogel, Micah     A155     01/05/2026     06/16/2026
8(A-D)     S2     WLG250-201     Spanish V     Kaluza, Marta J     A131     01/05/2026     06/16/2026
RC(A-Sp)     Y25-26     SLD130-4     Rolling Check     Staff, Residential Life         08/18/2025     06/16/2026
CC(A-Sp)     Y25-26     SLD120-4     Curfew Check     Staff, Residential Life         08/18/2025     06/16/2026
EVE(A-Sp)     Y25-26     SLD100-14     Residential Life     Doxey, Cameron         08/18/2025     06/16/2026



`
document.addEventListener('DOMContentLoaded', () => {

    let first_time_visiter = true;
    function setCookie(c_name, value, exdays) { var exdate = new Date(); exdate.setDate(exdate.getDate() + exdays); var c_value = escape(value) + ((exdays == null) ? "" : "; expires=" + exdate.toUTCString()); document.cookie = c_name + "=" + c_value; }

    function getCookie(c_name) { var c_value = document.cookie; var c_start = c_value.indexOf(" " + c_name + "="); if (c_start == -1) { c_start = c_value.indexOf(c_name + "="); } if (c_start == -1) { c_value = null; } else { c_start = c_value.indexOf("=", c_start) + 1; var c_end = c_value.indexOf(";", c_start); if (c_end == -1) { c_end = c_value.length; } c_value = unescape(c_value.substring(c_start, c_end)); } return c_value; }

    checkSession();

    function checkSession() {
        var c = getCookie("visited");
        if (c === "yes") {

            first_time_visiter = false
        } else {
            alert("Hello! It seems to be your first time using the site. Review the about page if you need help getting your schedule inputted.")
        }
        setCookie("visited", "yes", 365); // expire in 1 year; or use null to never expire
    }

    let has_submitted = false;
    document.getElementById('bottom-part').hidden = false;
    const colorMap = {
        0: "red",
        1: "darkorange",
        2: "yellow",
        3: "chartreuse",
        4: "green",
        5: "cyan",
        6: "blue",
        7: "purple",
        8: "pink",
        9: "brown"
    };


    const scaleFactor = 1.05;
    let currentScale = 1;
    let isPanning = false;
    let startPan = { x: 0, y: 0 };
    let translate = { x: -390, y: -1410 };
    const panzoomContainer = document.getElementById('panzoom-container');
    let baseWidth = 0, baseHeight = 0;
    const manualScaleFactor = 0.86;
    const svg = document.getElementById("mySvg");
    applyTransform()

    const image = document.getElementById('hallwayImage');
    document.getElementById("downloadMapBtn").addEventListener("click", downloadMapImage);

    function downloadMapImage() {
        const image = document.getElementById("hallwayImage");
        const svg = document.getElementById("mySvg");

        const svgData = new XMLSerializer().serializeToString(svg);
        const svgBlob = new Blob([svgData], { type: "image/svg+xml;charset=utf-8" });
        const svgUrl = URL.createObjectURL(svgBlob);

        const canvas = document.createElement("canvas");
        canvas.width = image.naturalWidth;
        canvas.height = image.naturalHeight;
        const ctx = canvas.getContext("2d");

        const imgObj = new Image();
        imgObj.crossOrigin = "anonymous";
        imgObj.src = image.src;

        imgObj.onload = () => {
            ctx.drawImage(imgObj, 0, 0, canvas.width, canvas.height);

            const svgImg = new Image();
            svgImg.crossOrigin = "anonymous";
            svgImg.src = svgUrl;

            svgImg.onload = () => {
                ctx.drawImage(svgImg, 0, 0, canvas.width, canvas.height);

                const link = document.createElement("a");
                link.download = "map_with_arrows.png";
                link.href = canvas.toDataURL("image/png");
                link.click();

                URL.revokeObjectURL(svgUrl);
            };
        };
    }



    function adjustSvgSize() {
        const w = image.naturalWidth || image.width || 0;
        const h = image.naturalHeight || image.height || 0;
        if (!w || !h) return;

        panzoomContainer.style.width = w + 'px';
        panzoomContainer.style.height = h + 'px';
        svg.style.width = w + 'px';
        svg.style.height = h + 'px';
        svg.setAttribute('viewBox', `0 0 ${w} ${h}`);
    }

    if (image.complete) {
        adjustSvgSize();
    } else {
        image.addEventListener('load', () => {
            adjustSvgSize();
        });
    }

    function applyTransform() {
        panzoomContainer.style.transform = `translate(${translate.x}px, ${translate.y}px) scale(${currentScale})`;
    }

    function zoomCanvas(factor, centerX, centerY) {
        const prevScale = currentScale;
        currentScale = Math.min(8, Math.max(0.2, currentScale * factor));
        if (currentScale === prevScale) return;
        if (centerX != null && centerY != null) {
            const rect = panzoomContainer.getBoundingClientRect();
            const offsetX = centerX - rect.left;
            const offsetY = centerY - rect.top;
            const scaleChange = currentScale / prevScale - 1;
            translate.x -= offsetX * scaleChange;
            translate.y -= offsetY * scaleChange;
        }
        applyTransform();
    }

    panzoomContainer.addEventListener('mousedown', (e) => {
        isPanning = true;
        startPan = { x: e.clientX - translate.x, y: e.clientY - translate.y };
        panzoomContainer.style.cursor = 'grabbing';
    });
    window.addEventListener('mousemove', (e) => {
        if (!isPanning) return;
        translate.x = e.clientX - startPan.x;
        translate.y = e.clientY - startPan.y;
        applyTransform();
    });
    window.addEventListener('mouseup', () => {
        if (isPanning) {
            isPanning = false;
            panzoomContainer.style.cursor = 'grab';
        }
    });
    panzoomContainer.style.cursor = 'grab';
    panzoomContainer.addEventListener('wheel', (e) => {
        e.preventDefault();
        const factor = e.deltaY < 0 ? scaleFactor : 1 / scaleFactor;
        zoomCanvas(factor, e.clientX, e.clientY);
    }, { passive: false });

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

    function getMousePosition(event) {
        const rect = panzoomContainer.getBoundingClientRect();
        const mouseX = (event.clientX - rect.left) / currentScale;
        const mouseY = (event.clientY - rect.top) / currentScale;
        return { mouseX, mouseY };
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

    function isStairNode(node) {
        return /Stair.*F[12]$/.test(node.name);
    }




    function getFullPathDescription(pathDetails, start) {
        //if start is true then the given path stuff is a start, if false, it is a end
        //0 should be start 1 end
        let val = 0;
        if (start === false) {
            val = 1
        };


        const fullPath = pathDetails.path[0].name + ' → ' + pathDetails.path[pathDetails.path.length - 1].name;
        if (pathDetails.info[val] !== null) {
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
        const { mouseX, mouseY } = getMousePosition(e);
        let tooltipVisible = false;

        labels.forEach(label => {

            if (
                mouseX >= label.x &&
                mouseX <= label.x + label.width &&
                mouseY >= label.y &&
                mouseY <= label.y + label.height
            ) {
                tooltip.style.display = 'block';
                tooltip.style.left = `${e.clientX + window.scrollX + 5}px`;
                tooltip.style.top = `${e.clientY + window.scrollY + 5}px`;

                let tooltipContent = `<strong>${label.name}</strong><br/>`;
                tooltipContent += getFullPathDescription(label.pathDetails, false);
                let black_if_yellow = "";
                if (label.color === "yellow" || label.color === "cyan" || label.color === "chartreuse") {
                    black_if_yellow = "color: black;"
                }

                tooltip.innerHTML = `<div style="${black_if_yellow}">${tooltipContent}</div>`;

                tooltip.style["border"] = "1px solid " + label.color;
                tooltip.style["backgroundColor"] = label.color;
                tooltip.style["borderRadius"] = "10px";

                tooltipVisible = true;
            }
        });

        if (!tooltipVisible) {
            tooltip.style.display = 'none';
        }
    }


    adjustSvgSize();
    window.addEventListener('resize', () => {
        if (has_submitted) {
            arrowFetch();
        }


    });

    document.getElementById("zoomIn").addEventListener("click", () => {
        const mapRect = document.getElementById('map').getBoundingClientRect();
        zoomCanvas(scaleFactor, mapRect.left + mapRect.width / 2, mapRect.top + mapRect.height / 2);
    });
    document.getElementById("viewTog").addEventListener("click", () => toggleMinimize());
    document.getElementById("zoomOut").addEventListener("click", () => {
        const mapRect = document.getElementById('map').getBoundingClientRect();
        zoomCanvas(1 / scaleFactor, mapRect.left + mapRect.width / 2, mapRect.top + mapRect.height / 2);
    });

    document.addEventListener('keydown', (e) => {
        if (e.key === '+') {
            const mapRect = document.getElementById('map').getBoundingClientRect();
            zoomCanvas(scaleFactor, mapRect.left + mapRect.width / 2, mapRect.top + mapRect.height / 2);
        } else if (e.key === '-') {
            const mapRect = document.getElementById('map').getBoundingClientRect();
            zoomCanvas(1 / scaleFactor, mapRect.left + mapRect.width / 2, mapRect.top + mapRect.height / 2);
        }
    });

    document.getElementById('scheduleForm').addEventListener('submit', function (e) {
        has_submitted = true;
        const example = document.getElementById("exampleSched").value;
        console.log(example);
        if (example != "no_ex") {
            let input = document.getElementById("scheduleInput");
            if (example == "ex1") {
                input.value = ex1;
            } else if (example == "ex2") {
                input.value = ex2;

            } else if (example == "ex3") {
                input.value = ex3;

            } else {
                console.error("Invalid example key given of '" + example + "'")
            }
        }
        e.preventDefault();

        arrowFetch();



    });
    function arrowFetch() {
        applyTransform();
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
                console.log("Here is the json recieved: ", json);
                if (json.status == 1) {
                    document.getElementById('error_message').innerHTML = `There was an error: ${json.error_message}. Hint: checking the <a id="error_message_link" href="/about">About Page</a> might help you`;
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

                    const pathDetails = {
                        path: path["nodes"],
                        startName: path["nodes"][0].name,
                        endName: path["nodes"][path["nodes"].length - 1].name,
                        info: path["info"],
                    };

                    for (let i = 1; i < path["nodes"].length; i++) {
                        const container = document.getElementById('hallwayImage');
                        const width = baseWidth || container.naturalWidth || container.getBoundingClientRect().width;
                        const height = baseHeight || container.naturalHeight || container.getBoundingClientRect().height;

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

                        const x1 = (node1.x + xOffset) * manualScaleFactor;
                        const y1 = (node1.y + yOffset) * manualScaleFactor;
                        const x2 = (node2.x + xOffset) * manualScaleFactor;
                        const y2 = (node2.y + yOffset) * manualScaleFactor;
                        const isStair = isStairNode(node1) && isStairNode(node2);
                        arrows.push({
                            x1Pct: x1 / width,
                            y1Pct: y1 / height,
                            x2Pct: x2 / width,
                            y2Pct: y2 / height,
                            color: colorMap[curnum],
                            name: node2.name,
                            pathDetails,
                            num: i - 1,
                            type: i === 1 ? 'start' : i === path["nodes"].length - 1 ? 'end' : 'mid',
                            isStairTransition: isStair
                        });

                    }
                });

                const scheduleBox = document.getElementById('scheduleBox');
                scheduleBox.innerHTML = '';

                curday.forEach((path, curnum) => {
                    const startName = path["nodes"][0].name;
                    const endName = path["nodes"][path["nodes"].length - 1].name;
                    const line = document.createElement('div');
                    line.className = 'schedule-line';

                    const pill = document.createElement('div');
                    pill.className = 'schedule-pill';
                    pill.style.backgroundColor = colorMap[curnum] || 'white';

                    const text = document.createElement('span');
                    text.textContent = `${startName} → ${endName}`;

                    line.appendChild(pill);
                    line.appendChild(text);
                    scheduleBox.appendChild(line);
                });


                adjustSvgSize();
                document.getElementById('error_message').innerHTML = "";
                redrawArrows();
                //document.getElementById('bottom-part').hidden = true;
                //document.getElementById('map').style.height = '100%';
                //this seems to be confusing ppl, will be removing for now.
                //if added in future use toggleMinimize() to do it also
            })

    }




    const arrows = [];
    const labels = [];
    const tooltip = document.getElementById('tooltip');

    svg.addEventListener('mousemove', handleMouseMove);
    svg.addEventListener('mouseleave', () => tooltip.style.display = 'none');

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


    function redrawArrows() {
        labels.length = 0;

        const container = document.getElementById('hallwayImage');
        if (!container) return console.error("hallwayImage container not found!");
        const width = baseWidth || container.naturalWidth || container.getBoundingClientRect().width;
        const height = baseHeight || container.naturalHeight || container.getBoundingClientRect().height;
        if (!width || !height) return console.error("Container has zero width or height:", width, height);
        const offset = 1320 / width;
        svg.innerHTML = '';

        let currentColor = null;
        let path = null;
        let d = '';

        const radius = 12 / currentScale;
        const shortenEnd = currentScale;
        let to_draw_arrowhead = 0;
        for (let i = 0; i < arrows.length; i++) {
            const cur = arrows[i];

            const x = cur.x1Pct * width / offset;
            const y = cur.y1Pct * height / offset;

            const nextX = cur.x2Pct * width / offset;
            const nextY = cur.y2Pct * height / offset;

            if (cur.isStairTransition) {
                drawCircle(x, y, cur.color);
                drawCircle(nextX, nextY, cur.color);

                function makeText(x, y, text) {
                    const SVG_NS = "http://www.w3.org/2000/svg";

                    const rect = document.createElementNS(SVG_NS, "rect");
                    rect.setAttribute("x", x + 5);
                    rect.setAttribute("y", y - 10);
                    rect.setAttribute("width", 120);
                    rect.setAttribute("height", 30);
                    rect.setAttribute("fill", cur.color);
                    rect.setAttribute("fill-opacity", "0.5");

                    svg.appendChild(rect);

                    const label = document.createElementNS(SVG_NS, "text");
                    label.setAttribute("font-weight", "bold");
                    label.setAttribute("x", x + 10);
                    label.setAttribute("y", y + 5);
                    label.setAttribute("fill", "black");
                    label.setAttribute("font-size", "12");
                    label.textContent = text;
                    svg.appendChild(label);
                }
                if ((cur.name).includes("F1")) {
                    makeText(x, y, "Take stairs down");
                } else {
                    makeText(x, y, "Take stairs up");
                }

                if (path) {
                    path.setAttribute("d", d);
                    svg.appendChild(path);
                    path = null;
                    d = '';
                    currentColor = null;
                }
                continue;
            }
            else {
                if (cur.color !== currentColor) {
                    if (path) {
                        path.setAttribute("d", d);
                        path.setAttribute("stroke-width", 2)
                        svg.appendChild(path);
                    }

                    path = document.createElementNS("http://www.w3.org/2000/svg", "path");
                    path.setAttribute("fill", "none");
                    path.setAttribute("stroke", cur.color);
                    path.setAttribute("stroke-width", 2);
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


                if (to_draw_arrowhead == 0) {
                    drawArrowhead(x, y, nextX, nextY, cur.color, 1.3 / currentScale);
                    to_draw_arrowhead = 1;
                } else if (to_draw_arrowhead == 3) {
                    to_draw_arrowhead = 0;
                } else {
                    to_draw_arrowhead += 1;
                };


                if (cur.type === 'end') {
                    const fontSize = 12;
                    const paddingX = 8;
                    const paddingY = 4;
                    const text = createSvgElement("text", {
                        x: 0,
                        y: 0,
                        "font-size": fontSize,
                        "font-weight": "bold",
                        fill: "white"
                    });
                    text.textContent = cur.name;
                    svg.appendChild(text);

                    const bbox = text.getBBox();
                    svg.removeChild(text);
                    const rectWidth = bbox.width + paddingX * 2;
                    const rectHeight = bbox.height + paddingY * 2;

                    const rectX = nextX + 5;
                    const rectY = nextY - rectHeight / 2;

                    const rect = createSvgElement("rect", {
                        x: rectX,
                        y: rectY,
                        width: rectWidth,
                        height: rectHeight,
                        fill: cur.color,
                        stroke: "black",
                        "stroke-width": 1,
                        rx: 4,
                        ry: 4
                    });
                    svg.appendChild(rect);
                    let text_color = "white";
                    if (cur.color === "yellow" || cur.color === "cyan" || cur.color === "chartreuse") {
                        text_color = "black";
                    }
                    const finalText = createSvgElement("text", {
                        x: rectX + rectWidth / 2,
                        y: rectY + rectHeight / 2 + bbox.height / 3,
                        "text-anchor": "middle",
                        "font-size": fontSize,
                        "font-weight": "bold",
                        fill: text_color,
                    });
                    finalText.textContent = cur.name;
                    svg.appendChild(finalText);
                    labels.push({
                        x: rectX,
                        y: rectY,
                        width: rectWidth,
                        height: rectHeight,
                        color: cur.color,
                        name: cur.name,
                        pathDetails: cur.pathDetails
                    });

                }
            }
        }

        if (path) {
            path.setAttribute("d", d);
            path.setAttribute("stroke-width", 2)
            svg.appendChild(path);
        }
    }



    function drawArrowhead(x1, y1, x2, y2, color, scale) {
        const angle = Math.atan2(y2 - y1, x2 - x1);
        const arrowLength = 6 * (scale || 1);

        const tipX = x2;
        const tipY = y2;

        const baseX1 = tipX - arrowLength * Math.cos(angle - Math.PI / 14);
        const baseY1 = tipY - arrowLength * Math.sin(angle - Math.PI / 14);

        const baseX2 = tipX - arrowLength * Math.cos(angle + Math.PI / 14);
        const baseY2 = tipY - arrowLength * Math.sin(angle + Math.PI / 14);


        const darkerColor = darkenColor(color, 0.6);

        const line1 = createSvgElement("line", {
            x1: tipX,
            y1: tipY,
            x2: baseX1,
            y2: baseY1,
            stroke: darkerColor,
            "stroke-width": 1
        });

        const line2 = createSvgElement("line", {
            x1: tipX,
            y1: tipY,
            x2: baseX2,
            y2: baseY2,
            stroke: darkerColor,
            "stroke-width": 1
        });

        svg.appendChild(line1);
        svg.appendChild(line2);
    }

    function darkenColor(color, factor = 0.7) {

        const ctx = document.createElement("canvas").getContext("2d");
        ctx.fillStyle = color;
        let computed = ctx.fillStyle;

        function hexToRgb(hex) {

            hex = hex.replace(/^#/, '');

            if (hex.length === 3) {
                hex = hex.split('').map(c => c + c).join('');
            }

            const num = parseInt(hex, 16);
            return [
                (num >> 16) & 255,
                (num >> 8) & 255,
                num & 255,
            ];
        }

        let r, g, b;

        if (computed.startsWith('#')) {
            [r, g, b] = hexToRgb(computed);
        } else {
            const rgbMatch = computed.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)/);
            if (!rgbMatch) {
                return color;
            }
            [r, g, b] = rgbMatch.slice(1, 4).map(Number);
        }

        r = Math.max(0, Math.floor(r * factor));
        g = Math.max(0, Math.floor(g * factor));
        b = Math.max(0, Math.floor(b * factor));

        return `rgb(${r}, ${g}, ${b})`;
    }



});