use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;


fn shuffle () -> Vec<i32>{
    let mut rng = StepRng::new(2, 13);
    let mut irs = Irs::default();

    let mut shuffled_deck = vec![1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,6,6,6,6,7,7,7,7,8,8,8,8,9,9,9,9,10,10,10,10,11,11,11,11,12,12,12,12,13,13,13,13];

   let _ = irs.shuffle(&mut shuffled_deck, &mut rng);
    shuffled_deck
}
fn deal (source: Vec<i32>, player_num: i32) -> Vec<i32> {
    let mut destination = Vec::new();
    let mut index= player_num - 1;
    let num_dealt = 52;

    while index < num_dealt {
        destination.push(source[index as usize]);
        index += 2;
    }
return destination;
}

fn play (mut p1_hand: Vec<i32>,mut p2_hand: Vec<i32>) {

   loop {
       let mut keep_card = 0;

       if p1_hand.is_empty() {
           println!("Player 1 is out of cards. Player 2 wins the war!");
           break;
       }
       if p2_hand.is_empty() {
           println!("Player 2 is out of cards. Player 1 wins the war!");
           break;
       }
        println!("Player 1 has {:?} cards left.", p1_hand.len());
        println!("Player 2 has {:?} cards left.", p2_hand.len());
        println!("Player 1 draws a {:?}",p1_hand[0]);
        println!("Player 2 draws a {:?}",p2_hand[0]);
        if p1_hand[0] > p2_hand[0] {
            println!("Player 1 takes both the {:?} and the {:?}.", p1_hand[0], p2_hand[0]);
            p1_hand.push(p2_hand.remove(0));
            keep_card = p1_hand.remove(0);
            p1_hand.push(keep_card);
        } else if p1_hand[0] < p2_hand[0] {
            println!("Player 2 takes both the {:?} and the {:?}.", p2_hand[0], p1_hand[0]);
            p2_hand.push(p1_hand.remove(0));
            keep_card = p2_hand.remove(0);
            p2_hand.push(keep_card);

        } else {
            println!("War!");
            let mut p1_waged_cards= 0;
            let mut p2_waged_cards = 0;
            let mut num_wars_waged = 0;
            let mut p1_war_card: isize  = 0;
            let mut p2_war_card: isize= 0;
            let mut keep_card = 0;

            loop {


                num_wars_waged += 1;

                //TODO debug multiple sequential war case
                println!("Num_wars_waged {:?}", num_wars_waged);


                if p1_hand.len() < (5 + (4*(num_wars_waged-1))) {
                    p1_war_card = (p1_hand.len()-1) as isize;
                    if p1_war_card == 0 {
                        p2_hand.push(p1_hand.remove(0));
                        break;
                    }
                    p1_waged_cards += p1_war_card - 1;
                    println!("WAR >> Player 1 has placed {:?} card(s) faced down in total and draws a {:?}", p1_waged_cards, p1_hand[(p1_war_card) as usize]);

                } else {
                     p1_war_card += 4;
                     p1_waged_cards += 3;
                    println!("WAR >> Player 1 has placed {:?} card(s) faced down in total and draws a {:?}", p1_waged_cards, p1_hand[(p1_war_card) as usize]);

                }
                if p2_hand.len() < (5 + (4*(num_wars_waged-1))) {
                     p2_war_card = (p2_hand.len()-1) as isize;
                    if p2_war_card == 0 {
                        p1_hand.push(p2_hand.remove(0));
                        break;
                    }
                     p2_waged_cards += p2_war_card - 1;
                    println!("WAR >> Player 2 has placed {:?} card(s) faced down in total and and draws a {:?}",p2_waged_cards ,p2_hand[(p2_war_card) as usize]);

                } else {
                     p2_war_card += 4;
                     p2_waged_cards += 3;
                    println!("WAR >> Player 2 has placed {:?} card(s) faced down in total and and draws a {:?}",p2_waged_cards, p2_hand[(p2_war_card) as usize]);
                }
                if p1_hand[(p1_war_card) as usize] > p2_hand[(p2_war_card) as usize] {
                    println!("WAR >> Player 1 wins the battle.");
                   while  p2_war_card >= 0 {
                       p1_hand.push(p2_hand.remove((p2_war_card)as usize));
                       p2_war_card -= 1;
                   }
                    while  p1_war_card >= 0 {
                        keep_card = p1_hand.remove((p1_war_card)as usize);
                        p1_hand.push(keep_card);
                        p1_war_card -= 1;
                    }
                    break;

                } else if p2_hand[(p2_war_card) as usize] > p1_hand[(p1_war_card) as usize] {
                    println!("WAR >> Player 2 wins the battle.");
                    while  p2_war_card >= 0 {
                        keep_card = p2_hand.remove((p2_war_card)as usize);
                        p2_hand.push(keep_card);
                        p2_war_card -= 1;
                    }
                    while  p1_war_card >= 0 {
                        p2_hand.push(p1_hand.remove((p1_war_card)as usize));
                        p1_war_card -= 1;
                    }
                    break;
                }
                else {
                    println!("WAR AGAIN!?! Yes! Let's go!");
                    println!("WAR >> Player 1 has {:?} cards left.", p1_hand.len());
                    println!("WAR >> Player 2 has {:?} cards left.", p2_hand.len());

                }
            }
        }
    }





}


fn main() {

    let full_deck = shuffle();
    let p1_deck: Vec<i32> = deal(full_deck.clone(), 1);
    let p2_deck: Vec<i32> = deal(full_deck.clone(), 2);
    play(p1_deck, p2_deck);


}
