from Node import *
from flask import Flask, render_template
import math
"""
ourapp = Server(31415, "Test")

@ourapp.app.route("/")
def hello_world():
  return render_template('home.html')

if __name__ == "__main__":
  ourapp.run_server()
"""
  
nodes = []
n0 = Node("test", [(1, 2), (2, 50)], 0)
n1 = Node("test", [(0, 2), (2, 3), (3, 3)], 1)
n2 = Node("test", [(0, 50), (1, 3), (3, 1)], 2)
n3 = Node("test", [(1, 3), (2, 1)], 3)
nodes.append(n0)
nodes.append(n1)
nodes.append(n2)
nodes.append(n3)

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

reset_nodes()
print(time_path(0,3))