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
                sh """
                docker run -d -p 8082:8081 hackk01/hospital_turn_notifications_api-server:latest
                """
            }
        }
    }
}
