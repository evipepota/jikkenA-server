input_file = './data/output.json'
output_file = './data/output_light.json'

def replace_text(input_file, output_file, target_word, replacement_word):
    with open(input_file, 'r') as file:
        text = file.read()

    for i in range(len(target_worda)):
        text = text.replace(target_word[i], replacement_word[i])

    with open(output_file, 'w') as file:
        file.write(text)

target_worda = ['"tag_name"', '"geotags"',
                '"date"', '"latitude"', '"longitude"', '"url"']
replacement_worda = ['"a"', '"b"', '"c"', '"d"', '"e"', '"f"']


replace_text(input_file, output_file, target_worda, replacement_worda)
