import random
import string

def print_random_characters(length):
    characters = string.ascii_letters + string.digits + string.punctuation
    random_string = ''.join(random.choice(characters) for _ in range(length))
    print(random_string)

if __name__ == "__main__":
    print_random_characters(256)