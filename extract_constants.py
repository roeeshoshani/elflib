import re
import pyclip
import sys
import os
import dataclasses
from collections import defaultdict

@dataclasses.dataclass
class Entry:
    variant_name: str
    comment: str

def snake_to_pascal_case(snake_case_str: str) -> str:
    return snake_case_str.replace("_", " ").title().replace(" ", "")

def make_variant_name(c_define_name: str, bitflags: bool) -> str:
    if bitflags:
        return c_define_name
    else:
        return snake_to_pascal_case(c_define_name)

def extract_constants(prefix: str, bitflags: bool):
    with open('/usr/include/elf.h', 'r') as f:
        lines = f.readlines()
    for rel_path in os.listdir('/home/sho/Documents/binutils-gdb/include/elf'):
        fullpath = '/home/sho/Documents/binutils-gdb/include/elf/' + rel_path
        with open(fullpath,'r') as f:
            lines += f.readlines()
    entries_of_value = defaultdict(list)
    for line in lines:
        pattern = f'^#define\s+{prefix}_([A-Za-z0-9_]+)\s+([^/]+)(?:\s*/\*(.*)\*)?'
        prefix_match = re.match(pattern, line)
        if prefix_match == None:
            continue

        variant_name, value_str, comment = prefix_match.groups()

        value_str = value_str.strip()

        if variant_name.lower() == 'num':
            continue

        if comment != None:
            comment = comment.strip()

        if variant_name[0].isdigit():
            if variant_name.lower().startswith('68k'):
                variant_name = f'm{variant_name}'
            elif variant_name.startswith('386'):
                variant_name = f'i{variant_name}'
            elif variant_name.startswith('390'):
                variant_name = f's{variant_name}'
            elif comment != None:
                comment_first_word = comment.split(' ')[0].lower()
                if comment_first_word in ['intel', 'motorola', 'renesas', 'freescale']:
                    variant_name = f'{comment_first_word}_{variant_name}'
                else:
                    raise Exception(f'variant name {repr(variant_name)} starts with a digit, comment is {comment}')
            else:
                raise Exception(f'variant name {repr(variant_name)} starts with a digit and has no comment')

        variant_name = make_variant_name(variant_name, bitflags)

        replacements = re.findall(f'{prefix}_([A-Z0-9a-z_]+)', value_str)
        for replacement in replacements:
            replacement_variant_name = make_variant_name(replacement, bitflags)
            replacement_value = None
            for value, entries in entries_of_value.items():
                for entry in entries:
                    if entry.variant_name == replacement_variant_name:
                        replacement_value = value
            value_str = value_str.replace(f'{prefix}_{replacement}', str(value))

        invalid_literals = re.findall(f'[0-9]+U', value_str)
        for invalid_literal in invalid_literals:
            value_str = value_str.replace(invalid_literal, invalid_literal[:-1])
        value = eval(value_str)

        already_have = False
        for entry in entries_of_value[value]:
            if entry.variant_name == variant_name:
                already_have = True
                break
        if not already_have:
            entries_of_value[value].append(Entry(variant_name, comment))

    res = ''
    for value, entries in entries_of_value.items():
        sep = '_OR_' if bitflags else 'Or'
        full_name = sep.join(entry.variant_name for entry in entries)
        if len(entries) > 1:
            # if any of the entries have a comment, then we must add a comment
            if any(entry.comment is not None for entry in entries):
                comment_contents = [entry.comment if entry.comment != None else entry.variant_name for entry in entries]
                comment_contents = [content.strip().removesuffix('.') for content in comment_contents]
                comment = ' Or '.join(comment_contents)
                res += f'/// {comment}\n'
        else:
            comment = entries[0].comment
            if comment != None:
                res += f'/// {comment}\n'
        if bitflags:
            res += f'const {full_name} = {hex(value)};\n'
        else:
            res += f'{full_name} = {hex(value)},\n'

    res = res.replace('Aarch64', 'AArch64')

    pyclip.copy(res)

def main():
    if len(sys.argv) not in [2,3]:
        print('usage: extract.py <prefix>')
    bitflags = len(sys.argv) > 2 and sys.argv[2] == 'bitflags'
    extract_constants(sys.argv[1], bitflags)

if __name__ == '__main__':
    main()
