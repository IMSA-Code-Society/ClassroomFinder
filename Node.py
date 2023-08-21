import math
from flask import Flask, render_template

class Node:
  def __init__(self, name, nodes, id, x, y):
    self.name = name
    self.nodes = nodes
    self.id = id
    self.dist = math.inf
    self.previous = None
    self.x = x
    self.y = y