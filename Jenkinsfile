pipeline {
    agent any
    environment {
        DOCKER_HUB_USER = credentials('dockerhub-credentials')
    }
    stages {
        stage('Checkout Code') {
            steps {
                echo 'Clonando el repositorio...'
                checkout scm
            }
        }
        stage('Pull Docker Image') {
            steps {
                echo 'Descargando la imagen desde Docker Hub...'
                sh """
                docker login -u ${DOCKER_HUB_USER_USR} -p ${DOCKER_HUB_USER_PSW}
                docker pull hackk01/hospital_turn_notifications_api-server:latest
                """
            }
        }
        stage('Deploy Application') {
            steps {
                echo 'Desplegando la aplicación en DigitalOcean...'
                script {
                    def containerName = "hospital_turn_notifications_api"
                    def isRunning = sh(script: "docker ps --filter 'name=${containerName}' --filter 'status=running' -q", returnStdout: true).trim()
                    
                    if (isRunning) {
                        echo "El contenedor '${containerName}' ya está corriendo. No se requiere ninguna acción adicional."
                    } else {
                        echo "El contenedor no está corriendo. Procediendo a eliminar y redeployar."
                        sh """
                        docker rm -f ${containerName} || true
                        docker run -d --name ${containerName} -p 8083:8081 hackk01/hospital_turn_notifications_api-server:latest
                        """
                        echo "Contenedor '${containerName}' desplegado correctamente."
                    }
                }
            }
        }
    }
}
