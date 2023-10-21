'''
Test the main function
'''
from src.sort import python_sort


def test_python_sort():
    '''
    Test the python_sort function
    '''
    data = [3, 2, 1]
    sorted_data = python_sort(data)
    assert sorted_data == [1, 2, 3]
