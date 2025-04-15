## cuda
- [Nvidia CUDA C++ Programming Guide](https://docs.nvidia.com/cuda/pdf/CUDA_C_Programming_Guide.pdf)
- [NVIDIA CUDA 编程指南 ](https://www.nvidia.cn/docs/IO/51635/NVIDIA_CUDA_Programming_Guide_1.1_chs.pdf)
- [CUDA 编程手册](https://github.com/HeKun-NVIDIA/CUDA-Programming-Guide-in-Chinese/tree/main)
- [【CUDA教學】平行計算](https://bayareanotes.com/cuda-tutorial/)
- [Nvidia CUDA](https://chenhh.gitbooks.io/parallel_processing/content/cuda/)
- [CUDA Programming](https://reference.wolfram.com/language/CUDALink/tutorial/Programming.html)
- [CUDA Tutorial](https://cuda-tutorial.readthedocs.io/en/latest/tutorials/tutorial01/)

### docker 環境
```sh
docker build -t cuda_image .
docker run -it -v ./project:/app --rm --name cuda_container cuda_image
docker exec -it cuda_container /bin/bash
```
> 1. 建立docker image
> 2. 背景執行docker container
> 3. 執行docker container