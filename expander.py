if __name__ == "__main__":
    main_file = "src/main.rs"

    src = []
    with open(main_file, "r") as f:
        for line in f:
            if len(line) >= 10 and line[-10:] == "// expand\n" and line[:3] == "mod":
                lib_name = line.split(" ")[1][:-1]
                src.append(f"pub mod {lib_name} {{\n")
                with open(f"src/{lib_name}.rs", "r") as f:
                    for line in f:
                        src.append("\t" + line)
                src.append("}\n")
            elif len(line) >= 10 and line[-10:] == "// ignore\n":
                continue
            else:
                src.append(line)

    for line in src:
        print(line, end="")
