/*
See: https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4etju/

OLD NOTES

Pour la partie 2, je pense qu'on doit partir des extrémités.  La
stratégie serait de chercher, de proche en proche, des préfixes qui
ne sont que des préfixes, cad qui n'apparaissent jamais à
l'intérieur ou à la fin d'une séquence de remplacement.

NOTE: Ce second critère est inutile! On a besoin de produire cette
chaîne comme préfixe, c'est tout.

Ce qu'on cherche, c'est à isoler un bout de la chaîne pour limiter
les candidats-molécules.  La solution, c'est de trouver un préfixe
qui ne peut être un élément que d'une séquence complète.

Par exemple, dans ma molécule cible:

CRnCaSiRnBSiRnFArTiBPTiT…

"C" est préfixe de plein de chaînes de remplacement, mais aucune ne
commence par R.  Il est donc impossible qu'une chaîne commençant
par C ait été développée dans un premier temps, puis remplacée à
partir de son deuxième caractère, etc: on ne parvien jamais à une
chaîne qui commence par un R majuscule.  C et R font donc partie de
la même chaîne.  Même chose avec le troisième caractère, n.  Aucun
remplacement encore ne commence par n (minuscule).  Par contre, les
quatrième et cinquième caractère, "Ca" sont préfixes de plein de
chaînes.  On s'arrête donc avant.

La séquence initiale CRn doit donc avoir été produite par une de
ces substitutions:

H => CRnAlAr
H => CRnFYFYFAr
H => CRnFYMgAr
H => CRnMgYFAr
N => CRnFAr
O => CRnFYFAr
O => CRnMgAr

La molécule de gauche est donc H, O ou N.

On sait donc deux choses:

Le développement se termine par une substitution dont le premier
terme est H, O ou N.  En fait, on n'apprend pas grand chose, parce
que c'est les trois seules possibilités à partir de e:

e => HF
e => NAl
e => OMg

L'autre truc intéressant, c'est qu'on a réduit la molécule cible:
on s'intéresse maintenant à produire uniquement:

CaSiRnBSiRnFArTiBPTiT…

Le nouveau préfixe est donc Ca:

Ca => CaCa
F => CaF
P => CaP
Si => CaSi

On a donc
[HON] puis [Ca|F|P|Si] etc.

Attention, à ce point, le préfixe diverge: dans l'option Si => CaSi,
on passe directement à RnBSi…  mais R n'est pas un préfixe: on ignore
donc Si => Casi.  Reste donc:

[HON] puis [Ca|F|P] etc.

Et on reprend à:

SiRnBSiRnFArTiBPTiT…

Idem avec SiRn: Si est un préfixe, mais pas Rn! Donc on prend SiRn en
entier, dans

Ca => SiRnFYFAr
Ca => SiRnMgAr
P => SiRnFAr

F n'est pas préfixe non plus, ni Mg: seul P => SiRnFAr est valide ici.

[HON] -> [Ca|F|P] -> P

Reste

TiBPTiT…

Deux remarques avant de dormir:

 1. Aucun remplacement ne commence par une minuscule.
 2. On devrait traverser les deux extrémités en même temps: on peut
 faire exactement la même chose avec les suffixes.  En anticipant bien,
 la même fct peut travailler dans les deux sens, et on se rejoint au milieu.

Ici, en poursuivant à la main, on peut commencer par:

e => OMg
(O => HP)
HPMg
H => HCa
donc HCaF

 */

use std::collections::HashSet;

fn main() {
    // Read input
    let mut replacements = vec![];
    let mut template = String::new();
    let mut part1 = true;
    let binding = std::fs::read_to_string("input").unwrap();
    for line in binding.split('\n') {
        if line.is_empty() {
            part1 = false;
        } else if part1 {
            let (search, replace) = line.split_once(" => ").unwrap();
            replacements.push((search, replace));
        } else {
            template = line.to_string();
            break;
        }
    }

    // Part 1
    let mut outcomes = HashSet::new();
    for (search, replace) in &replacements {
        let count = template.matches(search).count();
        let tpl = template.replace(search, "@");
        for i in 0..count {
            outcomes.insert(
                tpl.replacen("@", search, i)
                    .replacen("@", replace, 1)
                    .replace("@", search),
            );
        }
    }
    println!("Part 1: {}", outcomes.len());
    println!("Part 2: {}", part2(&template));
}

// Part 2: TODO reduce all non-overlapping patterns.

fn part2(
    template: &str,
 ) -> usize {
     let template = template.chars().collect::<Vec<char>>();

     let mut elements =0;
     let mut ar = 0;
     let mut rn = 0;
     let mut y = 0;
     for i in 0..template.len() {
         if template[i].is_uppercase() {
             elements += 1;
             if template[i] == 'R' && template[i+1] == 'n' {
                 rn += 1;
             } else if template[i] == 'A' && template[i+1] == 'r' {
                 ar+= 1;
             } else if template[i] == 'Y' {
                 y += 1;
             }
         }
     }
     (elements - (rn+ar) - 2*y - 1)
 }
// // Find a prefix
// let mut prefix = String::new();
// for i in 1..template.len() {
//     let cand = &template[0..i];
//     println!("Trying {cand}");
//     let mut is_prefix = false;
//     let mut is_only_prefix = true;
//     for (_, long) in replacements {
//         let matches = long
//             .match_indices(&cand)
//             .map(|(i, _)| i)
//             .collect::<Vec<usize>>();
//         if matches.is_empty() {
//             continue;
//         } else if matches.len() > 1 {
//             is_only_prefix = false;
//         }
//         if !matches.is_empty() && matches[0] == 0 {
//             is_prefix = true;
//         }
//     }
//     println!("prefix? {is_prefix}. Only? {is_only_prefix}");
//     if is_prefix && is_only_prefix {
//         prefix = cand.to_string();
//     } else if !is_prefix {
//         break;
//     }
// }
// println!("Prefix is {prefix}");
// ret
// }
