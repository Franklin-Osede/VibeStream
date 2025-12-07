FROM python:3.11-slim

WORKDIR /app

# Instalar dependencias del sistema para face_recognition (dlib)
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    libopenblas-dev \
    liblapack-dev \
    libx11-dev \
    libgtk-3-dev \
    python3-dev \
    && rm -rf /var/lib/apt/lists/*

# Copiar requirements e instalar dependencias Python
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copiar código
COPY . .

# Exponer puerto
EXPOSE 8004

# Variables de entorno por defecto
ENV PORT=8004
ENV DEBUG=false
ENV SIMILARITY_THRESHOLD=0.6
ENV DB_PATH=/app/facial_templates.db

# Comando para iniciar
CMD ["python", "main.py"]
