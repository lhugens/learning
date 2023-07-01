import torch
import torchvision
import torch.nn as nn
import torch.optim as optim

#x = torch.rand(3, 3)
#print(x)

#x = torch.tensor(2.0, requires_grad=True)
#y = 2 * x**3 + 3 * x**2 + 4 * x + 5
#
#y.backward()
#
#print(x.grad)

#x = torch.tensor(2.0, requires_grad=True)
#target = torch.tensor(10.0)
#
#y = 2 * x**3 + 3 * x**2 + 4 * x + 5
#
#loss = torch.abs(7 - target)
#
#loss.backward()
#
#print(x.grad)

#
#class Net(nn.Module):
#    def __init__(self):
#        super(Net, self).__init__()
#        self.fc1 = nn.Linear(10, 5)
#        self.fc2 = nn.Linear(5, 2)
#
#    def forward(self, x):
#        x = self.fc1(x)
#        x = torch.relu(x)
#        x = self.fc2(x)
#        return x
#
## Create an instance of the network
#net = Net()
#
## Perform forward pass
#input_data = torch.randn(10)
#print(input_data)
#output = net(input_data)
#print(output)

# TRAIN

class NeuralNetwork(nn.Module):
    def __init__(self):
        super(NeuralNetwork, self).__init__()
        self.fc1 = nn.Linear(10, 5)
        self.relu = nn.ReLU()
        self.fc2 = nn.Linear(5, 1)

    def forward(self, x):
        x = self.fc1(x)
        x = self.relu(x)
        x = self.fc2(x)
        return x

# Create an instance of the network
model = NeuralNetwork()

inputs = torch.randn(100, 10)
targets = torch.randn(100, 1)
criterion = nn.MSELoss()
optimizer = optim.SGD(model.parameters(), lr=0.01)

for epoch in range(100):

    outputs = model(inputs)
    loss = criterion(outputs, targets)

    optimizer.zero_grad()
    loss.backward()
    optimizer.step()



















































