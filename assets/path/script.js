document.body.addEventListener("mousedown", (current_click) => {
    document.getElementsByClassName("warning_box")[0].style.display = "none";
})
let canvas = document.getElementsByTagName("canvas")[0];
let context = canvas.getContext("2d");
const find_directions = () => {
    const starting_location = document.getElementById("start").value;
    const destination = document.getElementById("end").value;
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
            context.clearRect(0, 0, canvas.width, canvas.height);
            context.beginPath();

            const path = json.path;
            console.log("Drawing path:", path);

            for (let i = 1; i < path.length; i++) {
                console.log(`Drawing line from (${path[i - 1].x}, ${path[i - 1].y}) to (${path[i].x}, ${path[i].y})`);
                context.moveTo(path[i - 1]["x"], path[i - 1]["y"]);
                context.lineTo(path[i]["x"], path[i]["y"]);
                context.strokeStyle = "green";
                context.lineWidth = 5;
                context.stroke();
            }
        })
        .catch((err) => {
            console.log("Fetch error:", err);
        });


};
