pipeline {
    agent any
    environment {
        DOCKER_IMAGE = "tu_usuario_docker_hub/tu_imagen"
    }
    stages {
        stage('Checkout Code') {
            steps {
                echo 'Clonando el repositorio...'
                checkout scm
            }
        }
        stage('Build Docker Image') {
            steps {
                echo 'Construyendo la imagen Docker...'
                sh 'docker build -t $DOCKER_IMAGE .'
            }
        }
        stage('Push Docker Image') {
            steps {
                echo 'Subiendo la imagen a Docker Hub...'
                withDockerRegistry([credentialsId: 'docker-hub-credentials', url: '']) {
                    sh 'docker push $DOCKER_IMAGE'
                }
            }
        }
        stage('Deploy Application') {
            steps {
                echo 'Desplegando la aplicación...'
                sh 'docker run -d -p 8080:8080 $DOCKER_IMAGE'
            }
        }
    }
    post {
        always {
            echo 'Pipeline finalizado.'
        }
        success {
            echo 'Pipeline completado con éxito.'
        }
        failure {
            echo 'Hubo un error en el pipeline.'
        }
    }
}
