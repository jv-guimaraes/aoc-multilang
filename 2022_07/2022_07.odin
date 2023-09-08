package main

import "core:fmt"
import "core:strings"

input :: #load("input.txt", string)
matching_dirs: [dynamic]u32
space_to_be_deleted: u32 = 4294967295
root_dir_size: u32

str_to_u32 :: proc(str: string) -> (res: u32) {
    place_value: u32 = 1
    #reverse for v in str {
        res += (cast(u32) v - 48) * place_value
        place_value *= 10
    }
    return
}

build_tree :: proc(input: string) -> Dir {
    root := Dir{"root", [dynamic]^Node{}, nil}
    cwd := &root
    for line in strings.split_lines(input) {
        split_line := strings.split(line, " ")
        if line[0] == '$' {
            /*$ cd dir_name  |  ls*/
            switch split_line[1] {
                case "cd":
                    dir_name := split_line[2]
                    if dir_name == `/` {
                        cwd = &root
                    } else if dir_name == ".." {
                        cwd = cwd.parent
                    } else {
                        for i in 0..<len(cwd.children) {
                            node := cwd.children[i]
                            switch v in node {
                                case ^Dir:
                                    if dir_name == node.(^Dir).name {
                                        cwd = node.(^Dir);
                                    }
                                case ^File: continue
                            }
                        }
                    }
                case "ls":
                    continue
            }
        } else {
            /*8504156 c.dat  | dir d */
            switch split_line[0] {
                case "dir":
                    dir_name := split_line[1]
                    new_dir := create_dir(dir_name, cwd)
                    append(&cwd.children, create_node(new_dir))
                case: // File
                    file_size := str_to_u32(split_line[0])
                    file_name := split_line[1]
                    new_file := create_file(file_name, file_size)
                    append(&cwd.children, create_node(new_file))
            }
        }
    }
    return root
}

dir_size :: proc(node: ^Node) -> (size_res: u32) {
    switch v in node {
        case ^File: size_res += node.(^File).size
        case ^Dir:
            dir := node.(^Dir)
            for child in dir.children {
                size_res += dir_size(child)
            }
            if size_res <= 100_000 do append(&matching_dirs, size_res)
            if root_dir_size != 0  { // If root_dir_size was already calculated
                space_necessary := 30_000_000 - (70_000_000 - root_dir_size)
                if size_res >= space_necessary && size_res < space_to_be_deleted {
                    space_to_be_deleted = size_res
                }
            }
    }
    return
}

main :: proc() {
    root_dir := build_tree(input)
    root_node := create_node(&root_dir)
    root_dir_size = dir_size(root_node)
    
    // Part 1
    total: u32 = 0
    for size in matching_dirs do total += size
    fmt.println("Part 1:", total)

    // Part 2
    dir_size(root_node) // Used only for the side effects on the global variables
    fmt.println("Part 2:", space_to_be_deleted)

    // print_dir(root_node)
}