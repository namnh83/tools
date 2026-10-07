import random
import string

def generate_password(length, charset):
    return ''.join(random.choice(charset) for _ in range(length))

def main():
    while True:
        print("=== Password Generator ===\n")

        # 1. Password length
        while True:
            try:
                length = int(input("Password length (minimum 4): ").strip())
                if length >= 4:
                    break
                print("Please enter a number ≥ 4!\n")
            except ValueError:
                print("Please enter a valid number!\n")

        # 2. How many passwords
        while True:
            count_input = input("How many passwords? (1-100, default 10): ").strip()
            if count_input == "":
                count = 10
                break
            try:
                count = int(count_input)
                if 1 <= count <= 100:
                    break
                print("Please enter a number between 1 and 100!\n")
            except ValueError:
                print("Please enter a valid number!\n")

        # 3. Character types
        print("\nWhich character types should be used?")
        print("(Multiple selection – enter numbers separated by spaces, e.g. 1 2 4)\n")
        print("1 = Lowercase letters (a-z)")
        print("2 = Uppercase letters (A-Z)")
        print("3 = Numbers (0-9)")
        print("4 = Special characters (!@#$%^&* etc.)")

        while True:
            selection = input("\nSelection: ").strip().split()
            charset = ""

            if "1" in selection:
                charset += string.ascii_lowercase
            if "2" in selection:
                charset += string.ascii_uppercase
            if "3" in selection:
                charset += string.digits
            if "4" in selection:
                charset += "!@#$%^&*()_+-=[]{}|;:,.<>?"

            if charset:
                break
            print("Please select at least one valid option (1–4)!")

        # 4. Generate passwords
        print("\nGenerated passwords:")
        print("────────────────────────────────────")
        for i in range(1, count + 1):
            password = generate_password(length, charset)
            print(f"{i:>2}. {password}")
        print("────────────────────────────────────")

        # Ask again
        again = input("\nGenerate again? (y/n): ").strip().lower()
        if again != "y":
            print("\nPress Enter to exit...")
            input()
            break

        print()

if __name__ == "__main__":
    main()