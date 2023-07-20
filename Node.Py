from flask import Flask, render_template

class Node:
  def __init__(self, name, nodes):
    self.name = name
    self.nodes = nodes

class Server:
  def __init__(self, serverPort, name):
    self.serverPort = serverPort
    self.app = Flask(name)
    
  def run_server(self):
    self.app.run(host='0.0.0.0', debug=True, port=self.serverPort)

ourapp = Server(31415, "Test")

@ourapp.app.route("/")
def hello_world():
  return render_template('home.html')

if __name__ == "__main__":
  ourapp.run_server()
  
