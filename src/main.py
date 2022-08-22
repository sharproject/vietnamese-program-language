import os
import sys
import re

def check_coment(data):
    code = []
    for line in data:
        if line.startswith('#'):
            continue
        else:
            code.append(line)
    rcode = '\n'.join(code)
    translate_to_python(rcode)

def translate_to_python(data):
    pd = re.findall(r'in_ra_màn_hình:(.*)', data)
    for i in pd:
        try:
            if i.startswith('"') and i.endswith('"'):
                print(i[1:-1])
            else:
                print(eval(i))
        except ValueError:
            print('Invalid input')

def main(data, iseval=False):
    if data:
        if not iseval:
            if os.path.isfile(data):
                if data.endswith('.vipl'):
                    with open(data, 'r', encoding="utf-8") as f:
                        data = [l.strip()  for l in f.readlines()]
                        check_coment(data)
                else:
                    print('File is not a vipl file')
                    sys.exit(1)
            else:
                print('File does not exist')
                sys.exit(1)
        else:
            check_coment([a.strip() for a in data.split('\n')])


if __name__ == '__main__':
    try:
        data = sys.argv[1]
        main(data)
    except IndexError:
        while True:
            try:
                data = input('>>> ')
            except KeyboardInterrupt:
                print('\nBye')
                sys.exit(0)
            if data == 'exit()':
                break
                sys.exit(0)
            else:
                main(data, True)
        exit(0)