package main

import "core:fmt"
import "core:strings"

Dir :: struct {
    name: string,
    children: [dynamic]^Node,
    parent: ^Dir
}

File :: struct {
    name: string,
    size: u32
}

Node :: union {
    ^Dir,
    ^File
}

print_node :: proc(node: Node) {
    switch v in node {
        case ^Dir:
            dir := node.(^Dir)
            fmt.printf("(%s, %v, %s)",
                dir.name, children_to_str(dir.children), dir.parent.name)
        case ^File:
            file := node.(^File)
            fmt.println("(%s, %d)", file.name, file.size)
    }
}

@(private="file")
children_to_str :: proc(children: [dynamic]^Node) -> string {
    b := new(strings.Builder)
    fmt.sbprint(b, "[")
    for child in children {
        switch v in child {
            case ^Dir: fmt.sbprint(b, child.(^Dir).name, "| ")
            case ^File: fmt.sbprint(b, child.(^File).name, "| ")
        }
    }
    fmt.sbprint(b, "]")
    return strings.to_string(b^)
}

create_dir :: proc(name: string, parent: ^Dir) -> ^Dir {
    dir := new(Dir)
    dir.name, dir.parent = name, parent
    return dir
}

create_file :: proc(name: string, size: u32) -> ^File {
    file := new(File)
    file.name, file.size = name, size
    return file
}

@(private="file")
create_node_dir :: proc(elem: ^Dir)  -> ^Node{
    node := new(Node)
    node^ = elem
    return node
}

@(private="file")
create_node_file :: proc(elem: ^File) -> ^Node {
    node := new(Node)
    node^ = elem
    return node
}

create_node :: proc{create_node_dir, create_node_file}

print_dir :: proc(root_node: ^Node, indent := 0) {
    fmt.println(strings.repeat(" ", indent), "-", root_node.(^Dir).name)
    for curr_node in root_node.(^Dir).children {
        switch v in curr_node {
            case ^Dir:
                print_dir(curr_node, indent + 3)
            case ^File:
                file := curr_node.(^File)
                fmt.println(strings.repeat(" ", indent + 3),
                    "-", file.size, file.name)
        }
    }
}
