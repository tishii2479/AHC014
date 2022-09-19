import glob
import os
import subprocess

import tqdm
from PIL import Image
from reportlab.graphics import renderPM
from svglib.svglib import svg2rlg

print("txt -> svg")
out_files = sorted(glob.glob("out/*.txt"))
for out_file in tqdm.tqdm(out_files):
    name = os.path.splitext(os.path.basename(out_file))[0]
    with open("out/" + name + ".svg", "w") as f:
        subprocess.run(
            ["tools/target/release/vis", "tools/in/0011.txt", out_file],
            stdout=f,
        )

print("txt -> png")
svg_files = sorted(glob.glob("out/*.svg"))
for svg_file in tqdm.tqdm(svg_files):
    drawing = svg2rlg(svg_file)
    name = os.path.splitext(os.path.basename(svg_file))[0]
    print(svg_file, drawing, name)
    renderPM.drawToFile(drawing, "out/" + name + ".png", fmt="PNG")

print("png > gif")
png_files = sorted(glob.glob("out/*.png"))
images = list(map(lambda file: Image.open(file), png_files))
images[0].save(
    "visualize_annealing.gif",
    save_all=True,
    append_images=images[1:],
    duration=2000,
    loop=0,
)
