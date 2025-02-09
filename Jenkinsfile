pipeline {
    agent any
    environment {
        DOCKER_NEXUS_USER = credentials('nexus-credentials') // Credenciales configuradas en Jenkins para Nexus
        NEXUS_REGISTRY = '165.227.219.118:8085'
        IMAGE_NAME = 'hospital_turn_notifications_api-server'
        IMAGE_TAG = 'latest'
    }
    stages {
        stage('Checkout Code') {
            steps {
                echo 'Clonando el repositorio...'
                checkout scm
            }
        }
        stage('Initialize Buildx') {
    steps {
        echo 'Configurando Buildx...'
        sh """
        docker buildx create --name mybuilder --driver docker-container --use || echo "Builder already exists"
        docker buildx inspect --bootstrap
        """
    }
}
        stage('Build Docker Image') {
            steps {
                echo 'Construyendo la imagen Docker...'
                sh """
                export DOCKER_BUILDKIT=1
                docker buildx build --platform linux/amd64 -t ${NEXUS_REGISTRY}/docker-images/${IMAGE_NAME}:${IMAGE_TAG} --push .
                """
            }
        }
        stage('Push Docker Image to Nexus') {
            steps {
                echo 'Imagen ya subida en la etapa anterior.'
            }
        }
        stage('Deploy Application') {
            steps {
                echo 'Desplegando la aplicación en DigitalOcean...'
                script {
                    def containerName = "hospital_turn_notifications_api"
                    def isRunning = sh(script: "docker ps --filter 'name=${containerName}' --filter 'status=running' -q", returnStdout: true).trim()
                    
                    if (isRunning) {
                        echo "El contenedor '${containerName}' ya está corriendo. Eliminando y redeployando."
                        sh "docker rm -f ${containerName}"
                    }
                    
                    sh """
                    docker run -d --name ${containerName} -p 8083:8081 ${NEXUS_REGISTRY}/docker-images/${IMAGE_NAME}:${IMAGE_TAG}
                    """
                    echo "Contenedor '${containerName}' desplegado correctamente."
                }
            }
        }
    }
    post {
        always {
            echo 'Pipeline completado.'
        }
        success {
            echo 'Pipeline ejecutado con éxito.'
        }
        failure {
            echo 'Hubo un error en el pipeline.'
        }
    }
}
