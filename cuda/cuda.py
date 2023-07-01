import pycuda.autoinit
import pycuda.driver as cuda
from pycuda.compiler import SourceModule
import numpy as np

# CUDA kernel
mod = SourceModule("""
    __global__ void add_vectors(float *a, float *b, float *result, int n) {
        int idx = threadIdx.x + blockIdx.x * blockDim.x;
        if (idx < n) {
            result[idx] = a[idx] + b[idx];
        }
    }
""")

# Generate random input data
n = 100
a = np.random.randn(n).astype(np.float32)
b = np.random.randn(n).astype(np.float32)

# Allocate memory on the GPU
a_gpu = cuda.mem_alloc(a.nbytes)
b_gpu = cuda.mem_alloc(b.nbytes)
result_gpu = cuda.mem_alloc(b.nbytes)

# Copy input data to the GPU
cuda.memcpy_htod(a_gpu, a)
cuda.memcpy_htod(b_gpu, b)

# Launch the CUDA kernel
block_size = 32
grid_size = (n + block_size - 1) // block_size
func = mod.get_function("add_vectors")
func(a_gpu, b_gpu, result_gpu, np.int32(n), block=(block_size, 1, 1), grid=(grid_size, 1))

# Copy the result back to the CPU
result = np.empty_like(a)
cuda.memcpy_dtoh(result, result_gpu)

# Print the result
print(result)
