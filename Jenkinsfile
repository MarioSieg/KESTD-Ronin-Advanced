pipeline {
  agent any
    stages {
      stage('build') {
        steps {
          bat 'cargo build --release --all-features'
          bat 'cargo build --all-features'
        }
      }
      stage('test') {
        steps {
          bat 'cargo test'
          bat 'cargo clippy'
        }
      }
    }
}