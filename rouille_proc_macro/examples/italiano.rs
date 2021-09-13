rouille::ruggine! {
    esterno cassa rouille;

    utilizzare std::collections::Dizionario come Dizi;

    convenzione ValoreChiave {
        funzione scrivere(&auto, chiave: Catena, valore: Catena);
        funzione leggere(&auto, chiave: Catena) -> Risultato<Forse<&Catena>, Catena>;
    }

    statico mutevole DIZIONARIO: Forse<Dizi<Catena, Catena>> = Niente;

    struttura Calcestruzzo;

    raggiungimento ValoreChiave per Calcestruzzo {
        funzione scrivere(&auto, chiave: Catena, valore: Catena) {
            lascia dizi = pericoloso {
                DIZIONARIO.prendere_o_inserire_con(Default::default)
            };
            dizi.inserire(chiave, valore);
        }
        funzione leggere(&auto, chiave: Catena) -> Risultato<Forse<&Catena>, Catena> {
            se lascia Alcuni(dizi) = pericoloso { DIZIONARIO.come_rif() } {
                Bene(dizi.leggere(&chiave))
            } altrimenti {
                Arf("fetchez le dico".verso())
            }
        }
    }

    pubblico(cassa) funzione puo_essere(i: u32) -> Forse<Risultato<u32, Catena>> {
        se i % 2 == 1 {
            se i == 42 {
                Alcuni(Arf(Catena::da("merda")))
            } altrimenti {
                Alcuni(Bene(33))
            }
        } altrimenti {
            Niente
        }
    }

    asincrono funzione exemple() {
    }

    asincrono funzione exemple2() {
        exemple().attendere;
    }

    funzione principale() {
        lascia mutevole x = 31;

        corrisponde x {
            42 => {
                visualizza!("omelette du fromage")
            }
            _ => visualizza!("voila")
        }

        per i a 0..10 {
            let val = loop {
                fermare i;
            };

            entrambi x < val {
                x += 1;
            }

            x = se lascia Alcuni(resultat) = puo_essere(i) {
                resultat.scartare()
            } altrimenti {
                12
            };
        }

        // secondaire();
    }

    #[legale(codice_inaccessibile)]
    funzione secondaire() {
        merda!("oh non"); // for the true French experience
        calisse!("tabernacle"); // for friends speaking fr-ca
        oups!("fetchez la vache"); // in SFW contexts
    }
}
