from matplotlib import pyplot as PLT
import numpy as np

with open("out") as f:
	v = np.loadtxt(f, delimiter=":")
	#print(v)
	x = v[:,0]
	y = v[:,1]
	PLT.bar(x,y,align="center")
	PLT.savefig('out.png')
