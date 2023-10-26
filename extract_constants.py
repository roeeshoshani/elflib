import re
import pyclip
import sys
import os
import dataclasses
from collections import defaultdict

@dataclasses.dataclass
class Entry:
    variant_name: str
    const_variant_name: str
    comment: str

@dataclasses.dataclass
class ConstEntry:
    comment: str
    value: str

def snake_to_pascal_case(snake_case_str: str) -> str:
    return snake_case_str.replace("_", " ").title().replace(" ", "")

def make_variant_name(c_define_name: str, bitflags: bool) -> str:
    if bitflags:
        return c_define_name
    else:
        return snake_to_pascal_case(c_define_name)

def extract_constants(prefix: str, bitflags: bool):
    lines = []
    for rel_path in os.listdir('/home/sho/Documents/binutils-gdb/include/elf'):
        fullpath = '/home/sho/Documents/binutils-gdb/include/elf/' + rel_path
        with open(fullpath,'r') as f:
            lines += f.readlines()
    for rel_path in os.listdir('/home/sho/Documents/binutils-gdb/elfcpp'):
        fullpath = '/home/sho/Documents/binutils-gdb/elfcpp/' + rel_path
        with open(fullpath,'r') as f:
            lines += f.readlines()
    with open('/usr/include/elf.h', 'r') as f:
        lines += f.readlines()
    entries_of_value_str = defaultdict(list)
    all_names = set()

    unfixed_lines = lines
    lines = []
    i = 0
    while i < len(unfixed_lines):
        cur = unfixed_lines[i]
        i += 1
        while i < len(unfixed_lines):
            m = re.match(r'^(.*)\\s*$', cur)
            if m == None:
                break
            cur = m.groups()[0]
            cur += unfixed_lines[i]
            i += 1
        lines.append(cur)

    for line in lines:
        pattern = f'^#define\s+{prefix}_([A-Za-z0-9_]+)\s+([^/]+)(?:\s*/\*(.*)\*)?'
        prefix_match = re.match(pattern, line)
        if prefix_match == None:
            pattern = f'^#define\s+OLD_{prefix}_([A-Za-z0-9_]+)\s+([^/]+)(?:\s*/\*(.*)\*)?'
            prefix_match = re.match(pattern, line)
            if prefix_match == None:
                pattern = f'^\s*{prefix}_([A-Za-z0-9_]+)\s*=\s*(.*)$'
                prefix_match = re.match(pattern, line)
                if prefix_match == None:
                    continue
                variant_name, value_str = prefix_match.groups()
                comment = None
            else:
                variant_name, value_str, comment = prefix_match.groups()
                variant_name = 'OLD_' + variant_name
        else:
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
                if comment_first_word in ['intel', 'motorola', 'renesas', 'freescale', 'wdc', 'nxp']:
                    variant_name = f'{comment_first_word}_{variant_name}'
                else:
                    raise Exception(f'variant name {repr(variant_name)} starts with a digit, comment is {comment}')
            else:
                raise Exception(f'variant name {repr(variant_name)} starts with a digit and has no comment')

        const_variant_name = make_variant_name(variant_name, True).upper()
        variant_name = make_variant_name(variant_name, bitflags)

        already_have = False
        for entry in entries_of_value_str[value_str]:
            if entry.variant_name == variant_name:
                already_have = True
                break
        if not already_have and variant_name not in all_names:
            all_names.add(variant_name)
            entries_of_value_str[value_str].append(Entry(variant_name, const_variant_name, comment))

    entries_of_value = defaultdict(list)
    for initial_value_str, entries in entries_of_value_str.items():
        value_str = initial_value_str
        m = re.fullmatch('(.*)//.*', value_str)
        if m != None:
            value_str = m.groups()[0]
        value_str = value_str.strip()
        if value_str.endswith(','):
            value_str = value_str[:-1]

        while True:
            replaced_anything = False
            replacements = re.findall(f'OLD_{prefix}_([A-Z0-9a-z_]+)', value_str)
            for base_replacement in replacements:
                replacement = 'OLD_' + base_replacement
                replacement_variant_name = make_variant_name(replacement, bitflags)
                replacement_value = None
                for sub_value, sub_entries in entries_of_value_str.items():
                    for sub_entry in sub_entries:
                        if sub_entry.variant_name == replacement_variant_name:
                            replacement_value = sub_value
                if replacement_value == None:
                    raise Exception(f'failed to replace {replacement} in variant {variant_name}')
                value_str = value_str.replace(f'OLD_{prefix}_{base_replacement}', str(replacement_value))
                replaced_anything = True

            replacements = re.findall(f'{prefix}_([A-Z0-9a-z_]+)', value_str)
            for replacement in replacements:
                replacement_variant_name = make_variant_name(replacement, bitflags)
                replacement_value = None
                for sub_value, sub_entries in entries_of_value_str.items():
                    for sub_entry in sub_entries:
                        if sub_entry.variant_name == replacement_variant_name:
                            replacement_value = sub_value
                if replacement_value == None:
                    raise Exception(f'failed to replace {replacement} in variant {variant_name}')

                value_str = value_str.replace(f'{prefix}_{replacement}', str(replacement_value))
                replaced_anything = True


            invalid_literals = re.findall(f'[0-9]+U', value_str)
            for invalid_literal in invalid_literals:
                value_str = value_str.replace(invalid_literal, invalid_literal[:-1])
                replaced_anything = True

            invalid_literals = re.findall(f'[0-9]+L', value_str)
            for invalid_literal in invalid_literals:
                value_str = value_str.replace(invalid_literal, invalid_literal[:-1])
                replaced_anything = True

            invalid_literals = re.findall(f'[0-9]+LL', value_str)
            for invalid_literal in invalid_literals:
                value_str = value_str.replace(invalid_literal, invalid_literal[:-2])
                replaced_anything = True
            if not replaced_anything:
                break

        value = eval(value_str)
        for entry in entries:
            entries_of_value[value].append(Entry(entry.variant_name, entry.const_variant_name, entry.comment))

    res = ''
    enum_consts = {}
    enum_variant_name_of_value = {}
    for value, entries in entries_of_value.items():
        if bitflags:
            for entry in entries:
                name = entry.variant_name
                comment = entry.comment
                if comment != None:
                    res += f'/// {comment}\n'
                res += f'const {name} = {hex(value)};\n'
        else:
            sep = 'Or'
            full_name = sep.join(entry.variant_name for entry in entries)
            if len(entries) > 1:
                full_name = '_' + full_name
                # if any of the entries have a comment, then we must add a comment
                if any(entry.comment is not None for entry in entries):
                    comment_contents = [entry.comment if entry.comment != None else entry.variant_name for entry in entries]
                    comment_contents = [content.strip().removesuffix('.') for content in comment_contents]
                    comment = ' Or '.join(comment_contents)
                    res += f'/// {comment}\n'
                for entry in entries:
                    const_name = entry.const_variant_name
                    if const_name in enum_consts:
                        if enum_consts[const_name].value != value:
                            raise Exception(f'multiple values for const name {const_name}: {[value, enum_consts[const_name].value]}')
                    enum_consts[const_name] = ConstEntry(entry.comment, value)
            else:
                comment = entries[0].comment
                if comment != None:
                    res += f'/// {comment}\n'
            res += f'{full_name} = {hex(value)},\n'
            enum_variant_name_of_value[value] = full_name

    if len(enum_consts) > 0:
        res += '}\n'
        res += 'impl REPLACE {\n'
        for const_name, entry in enum_consts.items():
            comment = entry.comment
            if comment != None:
                res += f'/// {comment}\n'
            variant_name = enum_variant_name_of_value[entry.value]
            res += f'pub const {const_name} : Self = Self::{variant_name};\n'
    res += '}\n'

    res = res.replace('Aarch64', 'AArch64')

    pyclip.copy(res)

def main():
    if len(sys.argv) not in [2,3]:
        print('usage: extract.py <prefix>')
    bitflags = len(sys.argv) > 2 and sys.argv[2] == 'bitflags'
    extract_constants(sys.argv[1], bitflags)

if __name__ == '__main__':
    main()
