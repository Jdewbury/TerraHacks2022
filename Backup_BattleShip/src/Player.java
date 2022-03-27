import java.util.Scanner;

public class Player {
    private Board board=new Board();
    private int numBoats=4;
    private Boat[] boats=new Boat[numBoats];
    private int name;

    public Player(int name){
        for (int i = 2; i < numBoats; i++) {
            boats[i]=new Boat(i);
        }
        this.name=name;
    }

    public boolean guess(int location,Player player2){
        if(player2.board.getValueAt(location)==0){return false;}
        return true;
    }

    public void attack(Player player2){
        Scanner user = new Scanner(System.in);
        System.out.println("player"+ name+"'s turn, where do you guess?");
        int location =Integer.parseInt(user.nextLine());
        if(guess(location,player2)){
            System.out.println("You hit boat number: "+player2.board.getBoatAt(player2.boats,location).getLength());
            player2.board.getBoatAt(player2.boats,location).setDamage(location);
        }

    }

    public void addBoat(Boat boat,Player player2){
        int[] temp=new int[boat.getLength()];
        Scanner user = new Scanner(System.in);
        System.out.println("Player"+name+" on which squares do you want the boat with length: "+boat.getLength());
        for (int i = 0; i < boat.getLength(); i++) {
            temp[i]= Integer.parseInt(user.nextLine());
        }
        boat.setLocation(temp);
        board.addBoat(boat);
    }

    public boolean isPlaying(){
        for (Boat boat:boats) {
            if(boat!=null&&boat.alive){return true;}
        }
        return false;
    }
}
