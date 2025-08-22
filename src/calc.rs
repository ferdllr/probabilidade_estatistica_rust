use std::collections::HashMap;

pub struct Calc;

impl Calc {
    pub fn variancia(dados: &[f64]) -> Option<f64> {
        if dados.is_empty() {
            return None;
        }
        let media = Calc::media_aritimetica(dados);
        let mut desv: Vec<f64> = Vec::new();
        for &dado in dados {
            let mut res = dado - media;
            res *= res;
            desv.push(res);
        }
        Some(Calc::media_aritimetica(&desv))
    }

    pub fn amplitude(dados: &[f64]) -> Option<f64> {
        if dados.is_empty() {
            return None;
        }
        let mut max = dados[0];
        let mut min = dados[0];

        for &dado in dados {
            if max < dado {
                max = dado;
            }
            if min > dado {
                min = dado;
            }
        }
        Some(max - min)
    }

    pub fn moda(dados: &[i32]) -> Option<i32> {
        let mut map = HashMap::new();
        let mut bigger = 0;
        let mut moda = None;
        for &dado in dados {
            *map.entry(dado).or_insert(0) += 1;
        }
        for (&n, &v) in &map {
            if v > bigger {
                bigger = v;
                moda = Some(n);
            }
        }
        if bigger <= 1 {
            return None;
        }
        moda
    }

    pub fn mediana(dados: &[f64]) -> f64 {
        let mut lista = dados.to_vec();
        lista.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if lista.len() % 2 != 0 {
            let pos = lista.len() / 2;
            lista[pos]
        } else {
            let pos1 = lista.len() / 2 - 1;
            let pos2 = lista.len() / 2;
            (lista[pos1] + lista[pos2]) / 2.0
        }
    }

    pub fn media_aritimetica(dados: &[f64]) -> f64 {
        let mut somatoria = 0.0;
        for dado in dados {
            somatoria += dado;
        }
        somatoria / dados.len() as f64
    }

    pub fn probabilidade(espaco_amostral: &[i32], condicao: &dyn Fn(i32) -> bool) -> f64 {
        let mut dados_favoraveis = 0;

        for dado in espaco_amostral {
            if condicao(*dado) {
                dados_favoraveis += 1;
            }
        }

        dados_favoraveis as f64 / espaco_amostral.len() as f64
    }
}
