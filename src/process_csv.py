'''
This script processes the movies.csv file and counts the number of words in the review column
'''
import resource
import time
import psutil
import pandas as pd


def count_words_in_review(csv_path):
    '''
    Counts the number of words in the 'review' column of the csv file
    '''
    movies_df = pd.read_csv(csv_path)
    review_word_count = movies_df['review'].str.split().apply(len).sum()
    return review_word_count

def main():
    '''
    Main function
    '''
    start_mem_usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    start_time = time.time()

    words_counted = count_words_in_review('../movies.csv')

    core_usage = psutil.cpu_percent(interval=1, percpu=True)
    num_cores = len(core_usage)
    total_usage = sum(core_usage)
    average_usage = total_usage / num_cores

    end_time = time.time()
    execution_time = end_time - start_time
    end_mem_usage = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss

    print(f"Words in review column: {words_counted/1000000:.1f} million")
    print(f"Time taken: {execution_time:.3f} seconds")
    print(f"Average CPU core usage: {average_usage:.3f}%")
    print(f"Memory usage: {end_mem_usage - start_mem_usage} kilobytes")


if __name__ == "__main__":
    main()
