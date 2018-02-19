import scipy
import numpy
import matplotlib.pyplot as plt

print("Generating graph: ")
data = numpy.loadtxt(open("run_log.csv", "rb"), delimiter=",",skiprows=1)

x = data[:,0]
y = data[:,1]

fig = plt.figure()
plt.plot(x,y)
plt.ylabel('value')
plt.xlabel('time (s)')
plt.show()
fig.savefig("graph.png")
quit()
