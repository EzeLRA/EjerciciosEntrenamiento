function sym(...args) {
    // Función para diferencia simétrica entre dos arreglos
    const symDiff = (arr1, arr2) => {
        let diff1 = arr1.filter(elem => !arr2.includes(elem));
        let diff2 = arr2.filter(elem => !arr1.includes(elem));
        return [...diff1, ...diff2];
    };
    
    // Eliminar duplicados dentro de cada arreglo
    let arraysUnicos = args.map(arr => [...new Set(arr)]);
    
    // Aplicar diferencia simétrica acumulativa
    return arraysUnicos.reduce((acumulador, actual) => symDiff(acumulador, actual), []);
}

console.log(sym([1, 2, 3], [5, 2, 1, 4]));
