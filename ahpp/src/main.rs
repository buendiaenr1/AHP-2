use std::io;
use std::process::Command;

fn mi_aplicacion() {
   
    println!("Proceso de Análisis Jerárquico (AHP)");
    println!("Este programa calcula los pesos relativos de criterios y alternativas usando AHP.");

    // Paso 1: Definir los criterios
    println!("\nIngrese los nombres de los criterios separados por comas:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");

    let input_clone = input.clone(); // Clonar el valor de input
    let criteria: Vec<&str> = input_clone.trim().split(',').map(|s| s.trim()).collect();
    input.clear(); // Limpiar el input para reutilizarlo

    // Paso 2: Comparar criterios por pares
    println!("\nComparando los criterios por pares...");
    let criteria_matrix = pairwise_comparison(&criteria);

    // Paso 3: Normalizar la matriz y calcular los pesos de los criterios
    let normalized_criteria = normalize_matrix(&criteria_matrix);
    let criteria_weights = calculate_weights(&normalized_criteria);

    // Paso 4: Calcular la consistencia de la matriz de criterios
    match calculate_consistency(&criteria_matrix) {
        Ok((ci, cr)) => {
            println!("\nConsistencia de la matriz de criterios:");
            println!("Índice de Consistencia (CI): {:.3}", ci);
            println!("Razón de Consistencia (CR): {:.3}", cr);
            if cr <= 0.1 {
                println!("La matriz de criterios es consistente.");
            } else {
                println!("Advertencia: La matriz de criterios NO es consistente (CR > 0.1).");
            }
        }
        Err(e) => eprintln!("Error al calcular la consistencia: {}", e),
    }

    println!("\nPesos relativos de los criterios:");
    for (i, &weight) in criteria_weights.iter().enumerate() {
        println!("{}: {:.3}", criteria[i], weight);
    }

    // Paso 5: Definir las alternativas
    println!("\nIngrese los nombres de las alternativas separadas por comas:");
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");

    let input_clone = input.clone(); // Clonar el valor de input nuevamente
    let alternatives: Vec<&str> = input_clone.trim().split(',').map(|s| s.trim()).collect();
    input.clear(); // Limpiar el input para reutilizarlo

    // Paso 6: Comparar alternativas por pares para cada criterio
    let mut alternative_weights = vec![vec![0.0; alternatives.len()]; criteria.len()];
    for (i, criterion) in criteria.iter().enumerate() {
        println!("\nComparando las alternativas para el criterio '{}':", criterion);
        let alt_matrix = pairwise_comparison(&alternatives);
        let normalized_alt = normalize_matrix(&alt_matrix);
        alternative_weights[i] = calculate_weights(&normalized_alt);

        // Calcular la consistencia de la matriz de alternativas
        match calculate_consistency(&alt_matrix) {
            Ok((ci, cr)) => {
                println!("\nConsistencia de la matriz de alternativas para '{}':", criterion);
                println!("Índice de Consistencia (CI): {:.3}", ci);
                println!("Razón de Consistencia (CR): {:.3}", cr);
                if cr <= 0.1 {
                    println!("La matriz de alternativas es consistente.");
                } else {
                    println!("Advertencia: La matriz de alternativas NO es consistente (CR > 0.1).");
                }
            }
            Err(e) => eprintln!("Error al calcular la consistencia: {}", e),
        }
    }

    // Paso 7: Calcular los puntajes globales de las alternativas
    let global_scores = calculate_global_scores(&alternative_weights, &criteria_weights);

    println!("\nPuntajes globales de las alternativas:");
    for (i, &score) in global_scores.iter().enumerate() {
        println!("{}: {:.3}", alternatives[i], score);
    }

    // Paso 8: Mostrar la mejor alternativa
    let best_index = global_scores
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
        .unwrap_or(0);

    println!(
        "\nLa mejor alternativa es: {} con un puntaje de {:.3}",
        alternatives[best_index], global_scores[best_index]
    );

}


// Función para comparar elementos por pares con una escala más granular
fn pairwise_comparison(elements: &[&str]) -> Vec<Vec<f64>> {
    let n = elements.len();
    let mut matrix = vec![vec![1.0; n]; n];

    // Definir la escala extendida
    let scale = [
        ("Extremadamente importante", 9.0),
        ("Entre extremadamente y mucho", 8.0),
        ("Mucho más importante", 7.0),
        ("Entre mucho y bastante", 6.0),
        ("Bastante importante", 5.0),
        ("Entre bastante y moderadamente", 4.0),
        ("Moderadamente importante", 3.0),
        ("Entre moderadamente e igual", 2.0),
        ("IGUAL de importantes", 1.0),
        ("Entre igual y moderadamente", 0.5),
        ("Moderadamente menos importante", 1.0 / 3.0),
        ("Entre moderadamente y bastante", 1.0 / 4.0),
        ("Bastante menos importante", 1.0 / 5.0),
        ("Entre bastante y mucho", 1.0 / 6.0),
        ("Mucho menos importante", 1.0 / 7.0),
        ("Entre mucho y extremadamente", 1.0 / 8.0),
        ("Extremadamente menos importante", 1.0 / 9.0),
    ];

    for i in 0..n {
        for j in (i + 1)..n {
            println!(
                "\n¿Cómo se compara '{}' con '{}'?",
                elements[i], elements[j]
            );
            println!("Elija una opción:");

            // Mostrar las opciones de la escala extendida
            for (idx, (label, _)) in scale.iter().enumerate() {
                println!("{}. {}", idx + 1, label);
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Error al leer la entrada");
            let choice: usize = input.trim().parse().unwrap_or(9); // Default a "IGUAL de importantes"

            // Obtener el valor correspondiente de la escala
            let value = scale.get(choice - 1).map(|(_, v)| *v).unwrap_or(1.0);

            matrix[i][j] = value;
            matrix[j][i] = 1.0 / value; // Simetría inversa
        }
    }

    matrix
}

// Función para normalizar la matriz de comparaciones por pares
fn normalize_matrix(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = matrix.len();
    let mut normalized = vec![vec![0.0; n]; n];

    for col in 0..n {
        let col_sum: f64 = matrix.iter().map(|row| row[col]).sum();
        for row in 0..n {
            normalized[row][col] = matrix[row][col] / col_sum;
        }
    }

    normalized
}

// Función para calcular los pesos relativos
fn calculate_weights(normalized_matrix: &[Vec<f64>]) -> Vec<f64> {
    let n = normalized_matrix.len();
    let mut weights = vec![0.0; n];

    for row in 0..n {
        let row_sum: f64 = normalized_matrix[row].iter().sum();
        weights[row] = row_sum / n as f64;
    }

    weights
}

// Función para calcular los puntajes globales de las alternativas
fn calculate_global_scores(
    alternative_weights: &[Vec<f64>],
    criteria_weights: &[f64],
) -> Vec<f64> {
    let num_alternatives = alternative_weights[0].len();
    let mut global_scores = vec![0.0; num_alternatives];

    for i in 0..num_alternatives {
        for (j, &weight) in criteria_weights.iter().enumerate() {
            global_scores[i] += alternative_weights[j][i] * weight;
        }
    }

    global_scores
}

// Función para calcular el índice de consistencia (CI) y la razón de consistencia (CR)
fn calculate_consistency(matrix: &[Vec<f64>]) -> Result<(f64, f64), &'static str> {
    let n = matrix.len();
    let weights = calculate_weights(&normalize_matrix(matrix));

    // Calcular λ_max (valor propio máximo aproximado)
    let mut lambda_max = 0.0;
    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..n {
            sum += matrix[i][j] * weights[j];
        }
        lambda_max += sum / weights[i];
    }
    lambda_max /= n as f64;

    // Calcular el índice de consistencia (CI)
    let ci = (lambda_max - n as f64) / (n as f64 - 1.0);

    // Obtener el índice aleatorio (RI) según el tamaño de la matriz
    let ri = match n {
        1 => 0.0,
        2 => 0.0,
        3 => 0.58,
        4 => 0.90,
        5 => 1.12,
        6 => 1.24,
        7 => 1.32,
        8 => 1.41,
        9 => 1.45,
        10 => 1.49,
        _ if n > 10 => 1.49,
        _ => return Err("Tamaño de matriz no soportado"),
    };

    // Calcular la razón de consistencia (CR)
    let cr = ci / ri;

    Ok((ci, cr))
}



fn main() {
  
    // limpiar
    Command::new("cmd")
    .args(&["/C", "cls"])
    .status()
    .expect("Error al ejecutar el comando cls");

            // ***************
            mi_aplicacion();


    // Mantener la consola abierta después de salir
    Command::new("cmd")
            .args(&["/C", "cmd /k"])
            .status()
            .expect("Error al ejecutar el comando cmd /k");
          
}
