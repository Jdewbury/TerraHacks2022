import java.util.Scanner;

public class UI {
    boolean player1Win=true;
    boolean player2Win=true;
    public void run(){
        Player player1=new Player(1);
        Player player2=new Player(2);
        for (int i = 2; i < 4; i++) {
            player1.addBoat(new Boat(i),player2);
        }
        for (int i = 2; i < 4; i++) {
            player2.addBoat(new Boat(i),player1);
        }
        while (true){
            player1.attack(player2);
            if(!player2.isPlaying()){player1Win=true;break;}
            player2.attack(player1);
            if(!player1.isPlaying()){player2Win=true;break;}
        }
        if (player1Win){
            System.out.println("Player1 Wins!");
        }
        else if(player2Win){
            System.out.println("Player2 Wins!");
        }
    }


}
