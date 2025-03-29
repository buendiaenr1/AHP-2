# AHP-2
## e _ learning
## AHP_V2.exe    es la mejor opción  así como index_V2.HTML

Tomando como base la información generada en (Quadri Noorulhasan, y otros, 2019) en la Tabla 1 de dicho documento, 
con respecto a los factores críticos de éxito (CSFs por sus siglas en ingles) y tomando en cuenta las dimensiones de: 
Estudiante (los alumnos son considerados como los principales actores del sistema, el sistema y 
la asistencia sería más exitosa y valiosa si los estudiantes lo usan apropiadamente), 
Instructor (La actitud y el enfoque de los instructores son fundamentales para la eficiencia e implementación productiva del sistema, 
como la utilidad de las conferencias), Diseño y contenidos (Uso de herramientas para el aprendizaje y los contenidos claros y 
fáciles de usar), Tecnología y sistema (transmisión de la enseñanza y calidad del sistema, utilización de diferentes dispositivos), 
Servicio de gestión institucional (apoyo organizacional, disponibilidad de infraestructura, apoyo a los profesores, así como las cuestiones éticas y legales).

El archivo **_ahp4.xlsm_** se usa:

1. colocar los criterios
    - 1.a debe borrar el contenido de todas las demás hojas: cuant, norm, vp.
        - 1.a.1 cuant tiene los valores de la matriz inicial
        - 1.a.2 norm tiene la matriz normalizada
        - 1.a.3 vp es el vector propio resultante (valoración de cada criterio)
    - 1.b la hoja DeMaEnc sirve para transformar la matriz final a encuesta (proceso contrario)
2. crear encuesta
3. llenar encuesta (colocar * o X )
4. cuantificar ( se mostrará si el criterio es consistente )

El archivo **_encuesta.xlsx_** : es la encuesta usada para recolectar la información con base a los criterios mencionados.

Se tomó como referencia al español la tabla1 de (Quadri Noorulhasan, y otros, 2019)  modificada, ver listado en la parte inferior. 
Los factores críticos de éxito IOS2 e IOS3 se agregaron de acuerdo con (Peñalosa Castro & Castañeda Figueiras, 2008), 
tomando en cuenta que el mismo documento menciona que es suficiente con que se cumpla alguna de las tres interrelaciones mediante el teorema de equivalencia. 

**LISTADO** Dimensiones y los factores críticos de éxito para el análisis

**Dimensiones**	CSFs

**Estudiantes**	Actitud de los estudiantes hacia el estudio de cursos en línea (ATE)
Motivación de los estudiantes (SM)
    - Que es una habilidad del aprendizaje autorregulado (Peñalosa Castro & Castañeda Figueiras, 2008).
Autoeficacia general de los estudiantes en internet (GIS)
Interacción estudiante – con otros estudiantes (IOS)
Interacción Estudiante – profesor (IOS2)
Interacción Estudiante – contenido (IOS3)
Compromiso de los estudiantes con los estudios en línea (CTO)

**Instructores**	Actitud de los instructores hacia la enseñanza y dar cursos en línea (IAT)
Habilidades de los instructores en TIC (IIS)
Comunicación lingüística sencilla de los instructores (ELC)
Comentarios oportunos apropiados de los instructores

**Diseño y contenidos**	Actividad de aprendizaje interactivo del curso (ILA)
Diseño apropiado del curso (ACD)
Uso de instrucción multimedia en el curso (UMI)
Organización estilo usuario amistoso del curso (UFO)
Contenido comprensible del curso
Los contenidos del curso se eligen por la facilidad de medirlos y aplicar en pruebas (Peñalosa Castro & Castañeda Figueiras, 2008)

**Tecnología y sistema**	Sistema (plataforma) apropiado(a) (AS)
Facilidad de acceso al sistema (plataforma) (EoA)
Soporte técnico para usuarios del sistema (plataforma) (TSU)
Velocidad de internet buena (GIS)
Infraestructura tecnológica eficiente para usar el sistema (plataforma) (ETI)
Fiabilidad (seguridad) del sistema (plataforma) (R)

**Servicio de gestión institucional**	Disponibilidad de infraestructura institucional (IR)
Disponibilidad financiera institucional (FR)
Formación de usuarios institucional (SST)
Apoyo al profesorado por parte de la institución (SF)
Problemas éticos y legales con apoyo institucional (ELI)


El archivo _**Agregación.xlsm**_ crea un vector propio normalizado de un grupo de vectores propios normalizados agregados.

Tiene la función de unificar todos los vectores propios normalizados de todas las encuestas realizadas y formar un solo vector normalizado final.
Que representa los pesos de los criterios de los encuestados o expertos.

El archivo _**enrlibdanj.xls**_ tiene los vectores propios normalizados de todas las encuestas evaluadas, se copian por separado al archivo _Agregación.xlsm_ para finalizar.


El archivo **index.HTML** verifica el artículo http://www.pertanika.upm.edu.my/resources/files/Pertanika%20PAPERS/JSSH%20Vol.%2020%20(S)%20Dec.%202012/08_page129-142.pdf 


**Referencias**

Peñalosa Castro, E., & Castañeda Figueiras, S. (2008). Generación de conocimiento en la educación en línea: 
un modelo para el fomento de aprendizaje activo y autorregulado. Revista mexicana de investigación educativa, 
13(36), 249-281. Recuperado el 13 de Noviembre de 2020, de http://www.scielo.org.mx/pdf/rmie/v13n36/v13n36a11.pdf

Quadri Noorulhasan, N., Mohamed Rafik Noor, Q., Nasser, T., AbdulHafeez, M., Asadullah, S., Alhuseen O., A., . . . Fahad Mazaed, A. (2019). 
Evaluating critical success factors in implementing E-learning system using multi criteria decision-making. 
PLOS ONE, 15(5). doi:doi:10.1371/journal.pone.0231465
