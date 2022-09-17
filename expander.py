if __name__ == "__main__":
    main_file = "src/main.rs"

    src = []
    with open(main_file, "r") as f:
        for line in f:
            if len(line) >= 10 and line[-10:] == "// expand\n":
                lib_name = line.split(" ")[1][:-1]
                src.append(f"pub mod {lib_name} {{\n")
                with open(f"src/{lib_name}.rs", "r") as f:
                    for line in f:
                        src.append("\t" + line)
                src.append("}\n")
            else:
                src.append(line)

    for line in src:
        print(line, end="")
