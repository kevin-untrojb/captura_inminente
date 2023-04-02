/// La estructura Position representa una posición en X,Y.
#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}


impl Position {
    /// Crea una nueva instancia de `Position` con las coordenadas especificadas.
    ///
    /// # Argumentos
    ///
    /// * `x` - La coordenada x de la posición.
    /// * `y` - La coordenada y de la posición.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use position::Position;
    ///
    /// let pos = Position::new(3, 4);
    /// ```
    pub fn new(x:usize,y:usize) -> Position{
        Position{x,y}
    }

    /// Obtiene la diferencia en coordenadas (dx, dy) entre dos posiciones.
    ///
    /// # Argumentos
    ///
    /// * `other` - La otra posición a comparar con la actual.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use ajedrez::Position;
    ///
    /// let pos1 = Position::new(3, 4);
    /// let pos2 = Position::new(1, 2);
    /// let (dx, dy) = pos1.get_dx_dy(pos2);
    /// ```
    pub fn get_dx_dy(&self, other: Self) ->(usize, usize){
        let dx = (self.x as isize - other.x as isize).abs();
        let dy = (self.y as isize - other.y as isize).abs();
        (dx.try_into().unwrap(), dy.try_into().unwrap())
    }
}