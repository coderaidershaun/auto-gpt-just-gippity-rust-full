from decouple import config

def save_book_to_file(book_content):
    with open('output.txt', 'w') as file:
        file.write(book_content)

book_content = "This is the final content of the book."

save_book_to_file(book_content)