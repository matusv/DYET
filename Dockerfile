FROM python:3.9.15

WORKDIR /project

COPY requirements.txt requirements.txt
RUN pip3 install -r requirements.txt

COPY baselines3 /project/baselines3
RUN pip3 install /project/baselines3

COPY poi.ipynb /project/poi.ipynb

CMD ["jupyter-lab","--ip=0.0.0.0","--no-browser","--allow-root","--port","9999"]