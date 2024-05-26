import sys

# Check if there are command line arguments
if len(sys.argv) > 1:
    # Get the input from command line arguments
    input_string = ' '.join(sys.argv[1:])
    
    # Convert the input to capital case
    capital_case = input_string.upper()
    
    # Print the capital case string
    print(capital_case)
else:
    print("No input provided.")