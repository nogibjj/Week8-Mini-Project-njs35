'''
This is the main file for the Python sort algorithm.
'''
import random
import resource
import time
import psutil


def python_sort(input_data: list) -> list:
    '''
    Sorts a list of integers using the built-in Python sort function.
    '''
    return sorted(input_data)


def main():
    '''
    Main function for the Python sort algorithm.
    '''

    data = [random.randint(0, 1000000) for _ in range(10000000)]

    start_mem_usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    start_time = time.time()

    _ = python_sort(data)

    core_usage = psutil.cpu_percent(interval=1, percpu=True)
    num_cores = len(core_usage)
    total_usage = sum(core_usage)
    average_usage = total_usage / num_cores

    end_time = time.time()
    execution_time = end_time - start_time
    end_mem_usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss

    print(f"Total time taken: {execution_time:.5f} seconds")
    print(f"Average CPU core usage: {average_usage:.3f}%")
    print(f"Memory usage: {end_mem_usage - start_mem_usage} kilobytes")


if __name__ == "__main__":
    main()
