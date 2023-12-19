import cv2 as cv
import os
import sys
import numpy as np
from skimage import feature
from time import sleep
import sys

CHAR_MAP = " .'`^\"\\,:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"
if not os.path.exists(sys.path[0] + "/tmp"):
    os.mkdir(sys.path[0] + "/tmp")
skip = 0
vc = cv.VideoCapture(sys.path[0] + "/../assets/Howl's Moving Castle - Trailer.mp4")
cnt = 0
ratio = 2
# with open(sys.path[0] + f"/tmp/movie.txt", "w") as f:
while True:
    (ret, frame) = vc.read()
    cnt += 1
    if not ret:
        break
    if cnt < 400:
        continue

    frame = cv.cvtColor(frame, cv.COLOR_BGR2GRAY).astype(np.uint8)

    frame = cv.resize(frame, (1280 // ratio, 720 // ratio))
    for i in range(frame.shape[0]):
        for j in range(frame.shape[1]):
            val = frame.T[j][i] // 3
            print(CHAR_MAP[val if val <= 70 else 70], end="")
        print("")
    sleep(0.5)
    os.system("clear")
    # frame = feature.canny(frame)
    # frame_str = np.zeros_like(frame, dtype=str)
    # frame_str[frame == True] = "⬜"
    # frame_str[frame == False] = "⬛"
    # f.write("\n".join(["".join(frame_line) for frame_line in frame_str]) + "\n")

# with open(sys.path[0] + "/tmp/movie.txt", "r") as f:
#     # print(f.read())
#     # os.system("clear")
#     for line in f:
#         print(line)
#         os.system("clear")
