# DYET (Do You Even Test?)

To run dockerized poi.ipynb notebook:
1. `docker build -t jupyterlab:latest .`
2. `docker run --rm -it -p 9999:9999 jupyterlab:latest`
3. With the correct token output from docker run you will be able to connect to the notebook at 127.0.0.1:9999
