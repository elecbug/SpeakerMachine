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
                        string url = Uri.UnescapeDataString(context.Request.RawUrl)
                            .TrimStart('/')
                            .ReplaceUnicode();

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

                            try
                            {
                                Request find = requests[new Random().Next(requests.Count)];

                                Console.WriteLine("Title: " + find.Title);
                                Console.WriteLine("Author: " + find.Name);
                                Console.WriteLine("Description: " + find.Description);

                                Requests.Remove(Requests.Find(x => x.Name == find.Name && x.DateTime == find.DateTime)!);
                            }
                            catch
                            {
                                Console.WriteLine("Requests is empty");
                                break;
                            }

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

    public static class StringExtension
    {
        public static string ReplaceUnicode(this string oldString)
        {
            char[] result = oldString.ToCharArray();

            for (int i = 0; i + 6 < oldString.Length; i++)
            {
                if (oldString[i] == '/' && oldString[i + 1] == 'u')
                {
                    if (IsHex(oldString[i + 2]) &&
                        IsHex(oldString[i + 3]) &&
                        IsHex(oldString[i + 4]) &&
                        IsHex(oldString[i + 5]))
                    {
                        result[i] = '\\';
                    }
                }
            }

            return new string(result);
        }

        private static bool IsHex(char c)
        {
            return (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F');
        }
    }
}