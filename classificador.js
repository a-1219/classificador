let nome = "Mansa Musa"
let xp = Math.floor(Math.random()*10001)
let nvl = ["Ferro", "Bronze", "Prata", "Ouro", "Platina", "Ascendente", "Imortal", "Radiante"]

if (xp <= 1000) {
    nvl = nvl[0]
} else if (xp <= 2000) {
    nvl = nvl[1]
} else if (xp <= 5000) {
    nvl = nvl[2]
} else if (xp <= 7000) {
    nvl = nvl[3]
} else if (xp <= 8000) {
    nvl = nvl[4]
} else if (xp <= 9000) {
     nvl = nvl[5]
} else if (xp <= 10000) {
    nvl = nvl[6]
} else {
    nvl = nvl[7]
}

console.log(`O herói ${nome} está no nível ${nvl}`)
