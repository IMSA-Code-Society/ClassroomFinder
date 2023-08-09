from Node import *
from flask import Flask, render_template, jsonify, request, send_file
import math
from json import load
nodes = []
with open("nodes.json", mode="rt") as nodes_file:
  response = load(nodes_file)
  for node in response:
    node_object = Node(node["name"], node["neighbor_nodes"], node["id"], node["x"], node["y"])
    nodes.append(node_object)


Finder = Server(31415, "Test")

def name_to_id(name):
  for node in nodes:
    if node.name == name:
      return node.id

# n0 = Node("test", [(1, 2), (2, 50)], 0)
# n1 = Node("test", [(0, 2), (2, 3), (3, 3)], 1)
# n2 = Node("test", [(0, 50), (1, 3), (3, 1)], 2)
# n3 = Node("test", [(1, 3), (2, 1)], 3)
# nodes.append(n0)
# nodes.append(n1)
# nodes.append(n2)
# nodes.append(n3)

def reset_nodes():
  for i in range(len(nodes)):
    nodes[i].dist = math.inf
    nodes[i].previous = None

def time_path(start_id, end_id):
  temp_node = nodes.copy()
  temp_node[start_id].dist = 0
  while len(temp_node) > 0:
    nodes_index = find_closest_node(temp_node)
    closest_node = temp_node[nodes_index]
    temp_node.pop(nodes_index)
    for neighbor_node in closest_node.nodes:
      neighbor_index = get_index(temp_node, neighbor_node[0])
      if neighbor_index == None:
        continue
      neighbor_distance = closest_node.dist + neighbor_node[1]
      if neighbor_distance < temp_node[neighbor_index].dist:
        temp_node[neighbor_index].dist = neighbor_distance
        temp_node[neighbor_index].previous = closest_node.id
  retrace = [end_id]
  found_path = False
  while not found_path:
    previous_id = nodes[retrace[0]].previous
    if previous_id == None:
      found_path = True
    else:
      retrace.insert(0, previous_id)
  return retrace
  
def get_index(temp_node, id):
  for i in range(len(temp_node)):
    if id == temp_node[i].id:
      return i
  return None
    
def find_closest_node(temp_node):
  closest_node = None
  minimum = math.inf
  for i in range(len(temp_node)):
    if temp_node[i].dist < minimum:
      minimum = temp_node[i].dist
      closest_node = i
  return closest_node



@Finder.app.route("/")
def home_page():
  return render_template('home.html')

@Finder.app.route("/editor")
def editor():
  return render_template('editor.html')

@Finder.app.route("/save", methods=["POST"])
def save():
  with open("nodes.json", mode="wt") as nodes_file:
    nodes_file.write(str(request.data))
  response = {"status": 0}
  return jsonify(response)

@Finder.app.route("/image")
def image():
  return send_file('templates/imsa_hallway.jpg', mimetype="image/jpeg")

@Finder.app.route("/get_directions", methods=["POST"])
def directions():
  reset_nodes()
  start_room = name_to_id(request.json['start-room'])
  print(start_room)
  destination = name_to_id(request.json['destination'])
  print(destination)
  shortest_path = time_path(start_room, destination)
  path_json = []
  for index in shortest_path:
    x_coordinate = nodes[index].x
    y_coordinate = nodes[index].y
    node = {"x": x_coordinate, "y": y_coordinate}
    path_json.append(node)
  return jsonify(path_json)

if __name__ == "__main__":
  Finder.run_server()

