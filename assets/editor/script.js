document.addEventListener('DOMContentLoaded', () => {
  
  const svg = document.getElementById("mySvg");
  const SVG_NS = "http://www.w3.org/2000/svg";

  let nodes_array = [];
  let id_increment = 0;
  let mouse_state = true;
  let set_connection = null;
  const scaleFactor = 1.2;
  let currentScale = 1;
  const manualScaleFactor = 0.86;


  const image = document.getElementById('hallwayImage');

  let alreadyPrompted = false;
  svg.addEventListener("mousedown", (current_click) => {


    const rect = svg.getBoundingClientRect();
    let x = current_click.clientX - rect.left;
    let y = current_click.clientY - rect.top;
    let click_existing = false;

    for (let i = 0; i < nodes_array.length; i++) {
      let distance = Math.sqrt(Math.pow(nodes_array[i].x - x, 2) + Math.pow(nodes_array[i].y - y, 2));
      if (distance < 10) {
        click_existing = true;
        if (mouse_state) {
          set_connection = nodes_array[i];
          mouse_state = false;
        } else {
          distance = Math.sqrt(Math.pow(nodes_array[i].x - set_connection.x, 2) + Math.pow(nodes_array[i].y - set_connection.y, 2));
          nodes_array[i].neighbor_nodes.push([set_connection.id, distance]);
          const fromNode = nodes_array.find(n => n.id === set_connection.id);
          fromNode.neighbor_nodes.push([nodes_array[i].id, distance]);

          // Create line between the two
          const line = document.createElementNS(SVG_NS, "line");
          line.setAttribute("x1", nodes_array[i].x);
          line.setAttribute("y1", nodes_array[i].y);
          line.setAttribute("x2", set_connection.x);
          line.setAttribute("y2", set_connection.y);
          line.setAttribute("stroke", "green");
          line.setAttribute("stroke-width", 2);
          svg.appendChild(line);

          mouse_state = true;
        }
        break;
      }
    }

    if (click_existing) return;

    const circle = document.createElementNS(SVG_NS, "circle");
    circle.setAttribute("cx", x);
    circle.setAttribute("cy", y);
    circle.setAttribute("r", 2);
    circle.setAttribute("fill", "yellow");
    circle.setAttribute("stroke", "yellow");
    svg.appendChild(circle);

    const user_input = prompt("Node name: ");
    const label = document.createElementNS(SVG_NS, "text");
    label.setAttribute("x", x + 10);
    label.setAttribute("y", y + 5);
    label.setAttribute("fill", "orange");
    label.setAttribute("font-size", "6");
    label.textContent = user_input;
    svg.appendChild(label);

    nodes_array.push({
      x,
      y,
      name: user_input,
      id: id_increment++,
      neighbor_nodes: []
    });
  });
  function zoomCanvas(factor) {
    currentScale *= factor;
    const scaleStr = `scale(${currentScale})`;
    svg.style.transform = scaleStr;
    image.style.transform = scaleStr;
  }

  const save_nodes = () => {
    const formData = new FormData();
    const file = new Blob([JSON.stringify(nodes_array)], { type: "application/json" });
    formData.append("file", file, "nodes.json");

    fetch("/save", {
      method: "POST",
      body: formData
    })
      .then(response => response.json())
      .then(json => console.log(json))
      .catch(err => console.error("Upload error:", err));
  }


  function toggleMinimize() {

    if (document.getElementById('edit-bottom-part').hidden) {
      document.getElementById('edit-bottom-part').hidden = false;
      document.getElementById('map').style.height = '75%';
    } else {
      document.getElementById('edit-bottom-part').hidden = true;
      document.getElementById('map').style.height = '100%';
    }
  }

  document.getElementById("zoomIn").addEventListener("click", () => zoomCanvas(scaleFactor));
  document.getElementById("viewTog").addEventListener("click", () => toggleMinimize());
  document.getElementById("zoomOut").addEventListener("click", () => zoomCanvas(1 / scaleFactor));
  function adjustSvgSize() {
    const imageRect = document.getElementById('hallwayImage').getBoundingClientRect();
    svg.style.width = `${imageRect.width}px`;
    svg.style.height = `${imageRect.height}px`;
    svg.setAttribute("viewBox", `0 0 ${imageRect.width} ${imageRect.height}`);
  }
  
  function visualize_nodes() {
    fetch("/assets/nodes.json")
      .then(response => response.json())
      .then(data => {
        adjustSvgSize()
        console.log(data);

        svg.innerHTML = "";



        data.forEach(node => {
          node.neighbor_nodes.forEach(neighbor => {
            const neighborNode = data.find(n => n.id === neighbor[0]);
            if (neighborNode) {

              const line = document.createElementNS("http://www.w3.org/2000/svg", "line");
              line.setAttribute("x1", node.x);
              line.setAttribute("y1", node.y);
              line.setAttribute("x2", neighborNode.x);
              line.setAttribute("y2", neighborNode.y);
              line.setAttribute("stroke", "green");
              svg.appendChild(line);

            }
          });
        });
        data.forEach(node => {
          // Draw the node as a circle
          const circle = document.createElementNS(SVG_NS, "circle");
          circle.setAttribute("cx", node.x);
          circle.setAttribute("cy", node.y);
          circle.setAttribute("r", 2);
          circle.setAttribute("fill", "green");
          circle.setAttribute("stroke", "green");
          svg.appendChild(circle);

          // Draw the node label as text
          const label = document.createElementNS(SVG_NS, "text");
          label.setAttribute("x", node.x + 2);
          label.setAttribute("y", node.y + 2);
          label.setAttribute("fill", "orange");
          label.setAttribute("font-size", "6px");
          label.setAttribute("font-family", "Arial");
          label.textContent = node.id + node.name;
          svg.appendChild(label);
        });
      })
      .catch(err => {
        console.error("Failed to load nodes:", err);
      });
  }
  document.getElementById("save-nodes").addEventListener("click", () => save_nodes());
  document.getElementById("vis-nodes").addEventListener("click", () => visualize_nodes());
  document.getElementById("calc-dist").addEventListener("click", () => calculate_distance());
  function calculate_distance() {
    const id1 = parseInt(document.getElementById("node1").value);
    const id2 = parseInt(document.getElementById("node2").value);

    fetch("/assets/nodes.json")
      .then(response => response.json())
      .then(data => {
        const node1 = data.find(node => node.id === id1);
        const node2 = data.find(node => node.id === id2);

        if (node1 && node2) {
          const distance = Math.sqrt(Math.pow(node2.x - node1.x, 2) + Math.pow(node2.y - node1.y, 2));
          document.getElementById("distance-output").textContent = `Distance: ${distance.toFixed(2)} pixels`;
        } else {
          document.getElementById("distance-output").textContent = "Invalid node IDs.";
        }
      })
      .catch(err => {
        console.error("Failed to load nodes:", err);
        document.getElementById("distance-output").textContent = "Error loading nodes.";
      });
  }
  window.addEventListener('resize', () => {
    const scaleStr = `scale(${currentScale})`;
    svg.style.transform = scaleStr;
    image.style.transform = scaleStr;
    visualize_nodes();
  });


});
