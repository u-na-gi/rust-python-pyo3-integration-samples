import numpy as np

def my_numpy_function(x):
    # Perform a simple calculation using numpy
    np_array = np.array([x])  # Create a numpy array
    return np.sum(np_array).item()  # Calculate and return the sum of the numpy array
