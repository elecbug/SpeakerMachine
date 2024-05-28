using Packet;
using System.Net;
using System.Text;
using System.Text.Json;
using System.Web;

namespace SpeakerS
{
    public class Program
    {
        public static HttpListener? Listener { get; private set; }
        public static List<Request>? Requests { get; private set; }

        public static void Main(string[] args)
        {
            if (args.Length != 1)
            {
                Console.WriteLine("Enter the one argument [HTTP_PREFIX]");
                return;
            }

            Console.WriteLine("Start Speaker Machine...");

            Listener = new HttpListener();
            Listener.Start();
            Listener.Prefixes.Add(args[0]);

            if (File.Exists("reqs.json"))
            {
                using (StreamReader reader = new StreamReader("reqs.json"))
                {
                    Requests = JsonSerializer.Deserialize<List<Request>>(reader.ReadToEnd());
                }
            }
            else
            {
                using (StreamWriter writer = new StreamWriter("reqs.json"))
                {
                    Requests = new List<Request>();

                    writer.Write("[]");
                }
            }

            new Thread(() =>
            {
                while (true)
                {
                    HttpListenerContext context = Listener.GetContext();

                    if (context.Request.RawUrl != null)
                    {
                        string url = Uri.UnescapeDataString(context.Request.RawUrl).TrimStart('/');
                        Console.WriteLine("GET: " + url);

                        Request? request = JsonSerializer.Deserialize<Request>(url);

                        if (request != null)
                        {
                            byte[] msg = Encoding.UTF8.GetBytes("Submit the [" + request.Title + "]");

                            Requests!.Add(request);
                            
                            using (StreamWriter writer = new StreamWriter("reqs.json"))
                            {
                                string json = JsonSerializer.Serialize(Requests);
                                writer.Write(json);
                            }

                            context.Response.OutputStream.Write(msg);
                            context.Response.Close();
                        }
                    }
                }
            }).Start();

            while (true)
            {
                switch (Console.ReadLine()!.ToLower())
                {
                    case "get":
                        {
                            List<Request> requests 
                                = (Requests!.ToArray().Clone() as Request[])!.ToList();

                            List<Request> append = new List<Request>();

                            foreach (Request request in requests)
                            {
                                for (int i = 0; i < request.RR; i++)
                                {
                                    append.Add(new Request(request));
                                }
                            }

                            requests.AddRange(append);

                            Console.WriteLine(JsonSerializer.Serialize(requests));

                            Request find = requests[new Random().Next(requests.Count)];

                            Console.WriteLine(find.Title);
                            Console.WriteLine(find.Name);
                            Console.WriteLine(find.Description);

                            Requests.Remove(Requests.Find(x => x.Name == find.Name && x.DateTime == find.DateTime)!);

                            foreach (Request request in Requests)
                            {
                                request.RR++;
                            }

                            using (StreamWriter writer = new StreamWriter("reqs.json"))
                            {
                                string json = JsonSerializer.Serialize(Requests);
                                writer.Write(json);
                            }
                        }
                        break;
                }
            }
        }
    }
}